//! Read-only selection copying; no host operation or mutable table data.
use crate::TableRow;
use gtk::{gdk, gio, glib, prelude::*};

pub(crate) fn install(view: &gtk::ColumnView, selection: &gtk::SingleSelection) -> gtk::Popover {
    let group = gio::SimpleActionGroup::new();
    let copy = gio::SimpleAction::new("copy", None);
    copy.set_enabled(selection.selected_item().is_some());
    let (weak_view, weak_selection) = (view.downgrade(), selection.downgrade());
    copy.connect_activate(move |_, _| {
        if let (Some(view), Some(selection)) = (weak_view.upgrade(), weak_selection.upgrade()) {
            // Resolve the current selected object at invocation. Reordering,
            // updates or removal while a menu is open cannot copy stale rows.
            if let Some(object) = selection
                .selected_item()
                .and_downcast::<glib::BoxedAnyObject>()
            {
                view.clipboard()
                    .set_text(&object.borrow::<TableRow>().cells.join("\t"));
            }
        }
    });
    let weak_copy = copy.downgrade();
    selection.connect_selected_item_notify(move |selection| {
        if let Some(copy) = weak_copy.upgrade() {
            copy.set_enabled(selection.selected_item().is_some());
        }
    });
    group.add_action(&copy);
    view.insert_action_group("table", Some(&group));
    view.update_property(&[gtk::accessible::Property::Description(
        "Ctrl+C copies the selected row as tab-separated text. Copy selected row is also available from the context menu."
    )]);
    let item = gtk::Button::builder()
        .label("Copy selected row")
        .action_name("table.copy")
        .accessible_role(gtk::AccessibleRole::MenuItem)
        .build();
    item.update_property(&[gtk::accessible::Property::Label("Copy selected row")]);
    item.add_css_class("flat");
    let menu = gtk::Popover::builder()
        .has_arrow(false)
        .accessible_role(gtk::AccessibleRole::Menu)
        .child(&item)
        .build();
    menu.set_parent(view);
    let popup = menu.downgrade();
    item.connect_clicked(move |_| {
        if let Some(popup) = popup.upgrade() {
            popup.popdown();
        }
    });
    let weak_view = view.downgrade();
    menu.connect_closed(move |_| {
        if let Some(view) = weak_view.upgrade() {
            view.grab_focus();
        }
    });
    let click = gtk::GestureClick::new();
    click.set_button(3);
    click.set_propagation_phase(gtk::PropagationPhase::Capture);
    let popup = menu.downgrade();
    click.connect_pressed(move |gesture, _, x, y| {
        if let Some(popup) = popup.upgrade() {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            show(&popup, x, y);
        }
    });
    view.add_controller(click);
    let keys = gtk::EventControllerKey::new();
    keys.set_name(Some("seven-table-clipboard"));
    keys.set_propagation_phase(gtk::PropagationPhase::Capture);
    let (actions, popup, weak_view) = (group.downgrade(), menu.downgrade(), view.downgrade());
    keys.connect_key_pressed(move |_, key, _, modifiers| {
        use gdk::{Key, ModifierType as M};
        let held = modifiers
            & (M::CONTROL_MASK
                | M::SHIFT_MASK
                | M::ALT_MASK
                | M::SUPER_MASK
                | M::META_MASK
                | M::HYPER_MASK);
        if matches!(key, Key::c | Key::C) && held == M::CONTROL_MASK {
            if let Some(actions) = actions.upgrade() {
                actions.activate_action("copy", None);
            }
            return glib::Propagation::Stop;
        }
        if key == Key::Menu && held.is_empty() || key == Key::F10 && held == M::SHIFT_MASK {
            if let (Some(popup), Some(view)) = (popup.upgrade(), weak_view.upgrade()) {
                show(
                    &popup,
                    f64::from(view.width()) / 2.,
                    f64::from(view.height()) / 2.,
                );
            }
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    view.add_controller(keys);
    menu
}

fn show(menu: &gtk::Popover, x: f64, y: f64) {
    menu.set_pointing_to(Some(&gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
    menu.popup();
}
