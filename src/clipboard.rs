//! Bounded asynchronous text transfers. Preflight before changing the buffer.
use glib::translate::{ToGlibPtr, ToGlibPtrMut};
use gtk::{gio, glib, prelude::*};
use std::{cell::Cell, future::Future, rc::Rc, time::Duration};
const LIMIT: usize = 65536;
const MIME: &[&str] = &["text/plain;charset=utf-8", "UTF8_STRING", "text/plain"];
async fn text(stream: gio::InputStream) -> Result<String, String> {
    let result = async {
        let mut bytes = Vec::new();
        loop {
            let chunk = stream
                .read_bytes_future((LIMIT + 1 - bytes.len()).min(8192), glib::Priority::DEFAULT)
                .await
                .map_err(|_| "Transferred text could not be read".to_string())?;
            if chunk.is_empty() {
                break;
            }
            bytes.extend_from_slice(chunk.as_ref());
            if bytes.len() > LIMIT {
                return Err("Transferred text exceeds 64 KiB; the selection was preserved".into());
            }
        }
        let value = String::from_utf8(bytes)
            .map_err(|_| "Transferred text must be valid UTF-8".to_string())?;
        if value.contains('\0') {
            return Err("Transferred text contains NUL bytes; the selection was preserved".into());
        }
        Ok(value)
    }
    .await;
    let _ = stream.close_future(glib::Priority::DEFAULT).await;
    result
}
async fn clipboard_text(clipboard: gtk::gdk::Clipboard) -> Result<String, String> {
    let (stream, _) = clipboard
        .read_future(MIME, glib::Priority::DEFAULT)
        .await
        .map_err(|_| "Clipboard text is unavailable".to_string())?;
    text(stream).await
}
fn selection(buffer: &gtk::TextBuffer) -> Option<(i32, i32)> {
    buffer
        .selection_bounds()
        .map(|(a, b)| (a.offset(), b.offset()))
}
fn focused(view: &gtk::TextView) -> bool {
    view.root()
        .and_then(|root| root.focus())
        .is_some_and(|focus| focus == *view.upcast_ref::<gtk::Widget>() || focus.is_ancestor(view))
}
fn position(view: &gtk::TextView, x: f64, y: f64) -> Option<i32> {
    if !x.is_finite() || !y.is_finite() {
        return None;
    }
    let (x, y) = view.window_to_buffer_coords(gtk::TextWindowType::Widget, x as i32, y as i32);
    let mut iter = view.buffer().start_iter();
    // GTK always fills the nearest iterator. Its boolean only says whether
    // the point is over text; the high-level Option discards blank-area hits.
    // https://docs.gtk.org/gtk4/method.TextView.get_iter_at_location.html
    unsafe {
        gtk::ffi::gtk_text_view_get_iter_at_location(
            view.to_glib_none().0,
            iter.to_glib_none_mut().0,
            x,
            y,
        );
    }
    Some(iter.offset())
}

fn transfer(
    view: &gtk::TextView,
    notice: &gtk::Label,
    epoch: Rc<Cell<u64>>,
    pending: Rc<Cell<u64>>,
    at: Option<i32>,
    read: impl Future<Output = Result<String, String>> + 'static,
    drop: Option<gtk::gdk::Drop>,
) {
    if !view.is_editable() || !view.is_sensitive() || !focused(view) {
        if let Some(drop) = drop {
            drop.finish(gtk::gdk::DragAction::empty());
        }
        return;
    }
    let buffer = view.buffer();
    let revision = epoch.get();
    let bounds = selection(&buffer);
    let cursor = buffer.cursor_position();
    let serial = pending.get().wrapping_add(1);
    pending.set(serial);
    let (message, weak) = (notice.downgrade(), view.downgrade());
    glib::MainContext::default().spawn_local(async move {
        let result = glib::future_with_timeout(Duration::from_secs(3), read)
            .await
            .unwrap_or_else(|_| Err("Text transfer timed out; the selection was preserved".into()));
        let applied = (|| {
            let (Some(view), Some(notice)) = (weak.upgrade(), message.upgrade()) else {
                return false;
            };
            if pending.get() != serial {
                return false;
            }
            let buffer = view.buffer();
            if !view.is_mapped()
                || !focused(&view)
                || !view.is_editable()
                || !view.is_sensitive()
                || epoch.get() != revision
                || selection(&buffer) != bounds
                || buffer.cursor_position() != cursor
            {
                notice.set_text(
                    "The editor changed or lost focus before transfer completed; try again",
                );
                return false;
            }
            let value = match result {
                Ok(value) => value,
                Err(error) => {
                    notice.set_text(&error);
                    return false;
                }
            };
            if value.is_empty() {
                notice.set_text("Transferred data contains no text");
                return false;
            }
            let current = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
            let removed = if at.is_none() {
                buffer
                    .selection_bounds()
                    .map(|(a, b)| buffer.text(&a, &b, true).len())
                    .unwrap_or(0)
            } else {
                0
            };
            if current.len() - removed + value.len() > LIMIT {
                notice.set_text("Paste would exceed 64 KiB; the selection was preserved");
                return false;
            }
            buffer.begin_user_action();
            if let Some(offset) = at {
                let mut iter = buffer.iter_at_offset(offset);
                buffer.insert(&mut iter, &value);
                buffer.place_cursor(&iter);
            } else {
                buffer.delete_selection(true, true);
                buffer.insert_at_cursor(&value);
            }
            buffer.end_user_action();
            notice.set_text("Text pasted; edits remain local until saved");
            true
        })();
        if let Some(drop) = drop {
            // Never request deletion at the drag source, including on rejection.
            drop.finish(if applied {
                gtk::gdk::DragAction::COPY
            } else {
                gtk::gdk::DragAction::empty()
            });
        }
    });
}
pub(crate) fn install(view: &gtk::TextView, notice: &gtk::Label) {
    let epoch = Rc::new(Cell::new(0u64));
    let generation = epoch.clone();
    view.buffer()
        .connect_changed(move |_| generation.set(generation.get().wrapping_add(1)));
    let pending = Rc::new(Cell::new(0u64));
    let (revision, serial, message) = (epoch.clone(), pending.clone(), notice.downgrade());
    view.connect_paste_clipboard(move |view| {
        view.stop_signal_emission_by_name("paste-clipboard");
        if let Some(notice) = message.upgrade() {
            transfer(
                view,
                &notice,
                revision.clone(),
                serial.clone(),
                None,
                clipboard_text(view.clipboard()),
                None,
            );
        }
    });
    let click = gtk::GestureClick::new();
    click.set_button(2);
    click.set_propagation_phase(gtk::PropagationPhase::Capture);
    let (weak, message, revision, serial) = (
        view.downgrade(),
        notice.downgrade(),
        epoch.clone(),
        pending.clone(),
    );
    click.connect_pressed(move |gesture, _, x, y| {
        gesture.set_state(gtk::EventSequenceState::Claimed);
        let (Some(view), Some(notice)) = (weak.upgrade(), message.upgrade()) else {
            return;
        };
        view.grab_focus();
        if let Some(at) = position(&view, x, y) {
            transfer(
                &view,
                &notice,
                revision.clone(),
                serial.clone(),
                Some(at),
                clipboard_text(view.primary_clipboard()),
                None,
            );
        }
    });
    view.add_controller(click);
    // Replace the native unbounded destination with our owned COPY-only target.
    // Public controller enumeration leaves keyboard, IME and drag sources intact.
    let controllers = view.observe_controllers();
    let previous: Vec<gtk::EventController> = (0..controllers.n_items())
        .filter_map(|index| {
            controllers
                .item(index)
                .and_downcast::<gtk::EventController>()
        })
        .filter(|controller| {
            controller.is::<gtk::DropTarget>() || controller.is::<gtk::DropTargetAsync>()
        })
        .collect();
    for controller in previous {
        view.remove_controller(&controller);
    }
    let target = gtk::DropTargetAsync::new(
        Some(gtk::gdk::ContentFormats::new(MIME)),
        gtk::gdk::DragAction::COPY,
    );
    target.set_propagation_phase(gtk::PropagationPhase::Capture);
    target.connect_drag_enter(|_, _, _, _| gtk::gdk::DragAction::COPY);
    target.connect_drag_motion(|_, _, _, _| gtk::gdk::DragAction::COPY);
    let (weak, message) = (view.downgrade(), notice.downgrade());
    target.connect_drop(move |_, drop, x, y| {
        let (Some(view), Some(notice)) = (weak.upgrade(), message.upgrade()) else {
            return false;
        };
        let Some(at) = position(&view, x, y) else {
            return false;
        };
        view.grab_focus();
        let reader = drop.clone();
        transfer(
            &view,
            &notice,
            epoch.clone(),
            pending.clone(),
            Some(at),
            async move {
                let (stream, _) = reader
                    .read_future(MIME, glib::Priority::DEFAULT)
                    .await
                    .map_err(|_| "Dropped text is unavailable".to_string())?;
                text(stream).await
            },
            Some(drop.clone()),
        );
        true
    });
    view.add_controller(target);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_stream_accepts_unicode_and_rejects_overflow_invalid_utf8_and_nul() {
        let context = glib::MainContext::new();
        context
            .with_thread_default(|| {
                for (bytes, accepted) in [
                    ("λ 日本語".as_bytes().to_vec(), true),
                    (vec![b'x'; LIMIT], true),
                    (vec![b'x'; LIMIT + 1], false),
                    (vec![0xff], false),
                    (b"a\0b".to_vec(), false),
                ] {
                    let stream =
                        gio::MemoryInputStream::from_bytes(&glib::Bytes::from_owned(bytes.clone()));
                    let result = context.block_on(text(stream.clone().upcast()));
                    assert_eq!(result.is_ok(), accepted);
                    if accepted {
                        assert_eq!(result.unwrap().as_bytes(), bytes);
                    }
                    assert!(stream.is_closed());
                }
            })
            .unwrap();
    }
}
