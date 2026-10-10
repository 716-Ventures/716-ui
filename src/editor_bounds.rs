//! Preserve a selected replacement when GTK deletes before committing IME text.
//! Only the immediately preceding, otherwise isolated selected deletion can be
//! undone. Accepted insertions and intervening changes invalidate that receipt.
use gtk::{glib, prelude::*};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Deletion {
    before: String,
    after: String,
    insert: i32,
    bound: i32,
    offset: i32,
    revision: u64,
    rejected: bool,
}

fn settle(
    buffer: &gtk::TextBuffer,
    receipt: Rc<RefCell<Option<Deletion>>>,
    epoch: Rc<Cell<u64>>,
    applying: Rc<Cell<bool>>,
    composing: Rc<Cell<bool>>,
) {
    let weak = buffer.downgrade();
    // GTK closes its history group after end-user-action emission. Run
    // after that stack unwinds, before another input event can intervene.
    glib::idle_add_local_full(glib::Priority::HIGH, move || {
        if composing.get() {
            return glib::ControlFlow::Break;
        }
        let Some(item) = receipt.borrow_mut().take() else {
            return glib::ControlFlow::Break;
        };
        let Some(buffer) = weak.upgrade() else {
            return glib::ControlFlow::Break;
        };
        if item.rejected
            && epoch.get() == item.revision
            && buffer.can_undo()
            && buffer.text(&buffer.start_iter(), &buffer.end_iter(), true) == item.after
        {
            applying.set(true);
            buffer.undo();
            if buffer.text(&buffer.start_iter(), &buffer.end_iter(), true) == item.before {
                buffer.select_range(
                    &buffer.iter_at_offset(item.insert),
                    &buffer.iter_at_offset(item.bound),
                );
                // An empty user action discards the rejected deletion's
                // redo entry while retaining all earlier undo history.
                buffer.begin_user_action();
                buffer.end_user_action();
            }
            applying.set(false);
        }
        glib::ControlFlow::Break
    });
}

pub(crate) fn install(view: &gtk::TextView, notice: &gtk::Label) {
    let buffer = view.buffer();
    let composing = Rc::new(Cell::new(false));
    let revision = Rc::new(Cell::new(0u64));
    let action_start = Rc::new(Cell::new(None));
    let pending = Rc::new(RefCell::new(None::<Deletion>));
    let replay = Rc::new(Cell::new(false));
    let (epoch, start) = (revision.clone(), action_start.clone());
    buffer.connect_begin_user_action(move |_| start.set(Some(epoch.get())));
    let start = action_start.clone();
    buffer.connect_end_user_action(move |_| start.set(None));
    let (epoch, receipt) = (revision.clone(), pending.clone());
    buffer.connect_changed(move |buffer| {
        epoch.set(epoch.get().wrapping_add(1));
        let valid = receipt.borrow().as_ref().is_some_and(|item| {
            item.revision == epoch.get()
                && buffer.text(&buffer.start_iter(), &buffer.end_iter(), true) == item.after
        });
        if !valid {
            receipt.borrow_mut().take();
        }
    });
    let (epoch, start, receipt, applying) = (
        revision.clone(),
        action_start,
        pending.clone(),
        replay.clone(),
    );
    let active = composing.clone();
    buffer.connect_delete_range(move |buffer, a, b| {
        receipt.borrow_mut().take();
        if applying.get() || start.get() != Some(epoch.get()) || !buffer.enables_undo() {
            return;
        }
        let Some((selection_a, selection_b)) = buffer.selection_bounds() else {
            return;
        };
        if (a.offset(), b.offset()) != (selection_a.offset(), selection_b.offset()) {
            return;
        }
        let before = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), true)
            .to_string();
        if before.len() > 65536 {
            return;
        }
        let after = format!(
            "{}{}",
            buffer.text(&buffer.start_iter(), a, true),
            buffer.text(b, &buffer.end_iter(), true)
        );
        let next = epoch.get().wrapping_add(1);
        *receipt.borrow_mut() = Some(Deletion {
            before,
            after,
            insert: buffer.cursor_position(),
            bound: buffer.iter_at_mark(&buffer.selection_bound()).offset(),
            offset: a.offset(),
            revision: next,
            rejected: false,
        });
        settle(
            buffer,
            receipt.clone(),
            epoch.clone(),
            applying.clone(),
            active.clone(),
        );
    });
    let (receipt, epoch, applying, active) = (
        pending.clone(),
        revision.clone(),
        replay.clone(),
        composing.clone(),
    );
    view.connect_preedit_changed(move |view, text| {
        let ended = active.replace(!text.is_empty()) && text.is_empty();
        if ended {
            // Cancellation has no insertion signal. Restore a still-isolated
            // preedit deletion unless an accepted commit clears the receipt
            // before the idle callback runs.
            if let Some(item) = receipt.borrow_mut().as_mut() {
                item.rejected = true;
            }
            settle(
                &view.buffer(),
                receipt.clone(),
                epoch.clone(),
                applying.clone(),
                active.clone(),
            );
        }
    });
    let message = notice.downgrade();
    buffer.connect_insert_text(move |buffer, location, text| {
        let current = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
        if text.contains('\0')
            || current
                .len()
                .checked_add(text.len())
                .is_none_or(|n| n > 65536)
        {
            buffer.stop_signal_emission_by_name("insert-text");
            if let Some(item) = pending.borrow_mut().as_mut() {
                item.rejected = item.revision == revision.get()
                    && item.offset == location.offset()
                    && current == item.after;
            }
            settle(
                buffer,
                pending.clone(),
                revision.clone(),
                replay.clone(),
                composing.clone(),
            );
            if let Some(notice) = message.upgrade() {
                notice
                    .set_text("Document text is limited to 64 KiB; this insertion was not applied");
            }
        } else {
            pending.borrow_mut().take();
        }
    });
}
