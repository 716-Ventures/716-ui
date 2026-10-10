//! Bounded asynchronous plain-text paste. Preflight before deleting any selection.
use gtk::{gio, glib, prelude::*};
use std::{cell::Cell, rc::Rc, time::Duration};
const LIMIT: usize = 65536;
async fn text(clipboard: gtk::gdk::Clipboard) -> Result<String, String> {
    let (stream, _) = clipboard
        .read_future(
            &["text/plain;charset=utf-8", "UTF8_STRING", "text/plain"],
            glib::Priority::DEFAULT,
        )
        .await
        .map_err(|_| "Clipboard text is unavailable".to_string())?;
    let result = async {
        let mut bytes = Vec::new();
        loop {
            let chunk = stream
                .read_bytes_future((LIMIT + 1 - bytes.len()).min(8192), glib::Priority::DEFAULT)
                .await
                .map_err(|_| "Clipboard text could not be read".to_string())?;
            if chunk.is_empty() {
                break;
            }
            bytes.extend_from_slice(chunk.as_ref());
            if bytes.len() > LIMIT {
                return Err("Clipboard text exceeds 64 KiB; the selection was preserved".into());
            }
        }
        let value = String::from_utf8(bytes)
            .map_err(|_| "Clipboard text must be valid UTF-8".to_string())?;
        if value.contains('\0') {
            return Err("Clipboard text contains NUL bytes; the selection was preserved".into());
        }
        Ok(value)
    }
    .await;
    let _ = stream.close_future(glib::Priority::DEFAULT).await;
    result
}
fn selection(buffer: &gtk::TextBuffer) -> Option<(i32, i32)> {
    buffer
        .selection_bounds()
        .map(|(a, b)| (a.offset(), b.offset()))
}
pub(crate) fn install(view: &gtk::TextView, notice: &gtk::Label) {
    let epoch = Rc::new(Cell::new(0u64));
    let generation = epoch.clone();
    view.buffer()
        .connect_changed(move |_| generation.set(generation.get().wrapping_add(1)));
    let pending = Rc::new(Cell::new(0u64));
    let message = notice.downgrade();
    view.connect_paste_clipboard(move |view| {
        view.stop_signal_emission_by_name("paste-clipboard");
        if !view.is_editable() || !view.is_sensitive() || !view.has_focus() {
            return;
        }
        let buffer = view.buffer();
        let revision = epoch.get();
        let bounds = selection(&buffer);
        let cursor = buffer.cursor_position();
        let serial = pending.get().wrapping_add(1);
        pending.set(serial);
        let (epoch, pending, message, weak) = (
            epoch.clone(),
            pending.clone(),
            message.clone(),
            view.downgrade(),
        );
        let clipboard = view.clipboard();
        glib::MainContext::default().spawn_local(async move {
            let result = glib::future_with_timeout(Duration::from_secs(3), text(clipboard))
                .await
                .unwrap_or_else(|_| {
                    Err("Clipboard read timed out; the selection was preserved".into())
                });
            let (Some(view), Some(notice)) = (weak.upgrade(), message.upgrade()) else {
                return;
            };
            if pending.get() != serial {
                return;
            }
            let buffer = view.buffer();
            if !view.is_mapped()
                || !view.has_focus()
                || !view.is_editable()
                || !view.is_sensitive()
                || epoch.get() != revision
                || selection(&buffer) != bounds
                || buffer.cursor_position() != cursor
            {
                notice.set_text(
                    "The editor changed or lost focus before paste completed; paste again",
                );
                return;
            }
            let value = match result {
                Ok(value) => value,
                Err(error) => {
                    notice.set_text(&error);
                    return;
                }
            };
            if value.is_empty() {
                notice.set_text("Clipboard contains no text");
                return;
            }
            let current = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
            let removed = buffer
                .selection_bounds()
                .map(|(a, b)| buffer.text(&a, &b, true).len())
                .unwrap_or(0);
            if current.len() - removed + value.len() > LIMIT {
                notice.set_text("Paste would exceed 64 KiB; the selection was preserved");
                return;
            }
            buffer.begin_user_action();
            buffer.delete_selection(true, true);
            buffer.insert_at_cursor(&value);
            buffer.end_user_action();
            notice.set_text("Clipboard text pasted; edits remain local until saved");
        });
    });
}
