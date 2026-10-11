//! Exercise installed native callbacks and release the last multiline owner.
use gtk::{gdk, prelude::*};
use seven_sixteen_ui::{gtk, InputEvent, TextArea};
use std::{cell::RefCell, rc::Rc};

fn main() {
    gtk::init().unwrap();
    let area = TextArea::new("Draft", "First λ\n日本語");
    let events = Rc::new(RefCell::new(Vec::new()));
    let observed = events.clone();
    area.on_event(move |event| observed.borrow_mut().push(event));
    area.view.buffer().insert_at_cursor(" appended");
    let expected = area.value();
    assert_eq!(
        events.borrow().last(),
        Some(&InputEvent::Changed(expected.clone()))
    );
    events.borrow_mut().clear();
    // Invoke the installed application controller, preserving GTK's own
    // unrelated editing controllers and avoiding model iteration order.
    let model = area.view.observe_controllers();
    let keys = (0..model.n_items())
        .filter_map(|i| model.item(i).and_downcast::<gtk::EventControllerKey>())
        .filter(|keys| keys.name().as_deref() == Some("seven-textarea-submit"))
        .collect::<Vec<_>>();
    assert_eq!(keys.len(), 1);
    for key in &keys {
        key.emit_by_name::<bool>(
            "key-pressed",
            &[&gdk::Key::Return, &0u32, &gdk::ModifierType::empty()],
        );
    }
    assert!(
        events.borrow().is_empty(),
        "Plain Return unexpectedly submitted a draft"
    );
    for key in &keys {
        key.emit_by_name::<bool>(
            "key-pressed",
            &[&gdk::Key::Return, &0u32, &gdk::ModifierType::CONTROL_MASK],
        );
    }
    assert_eq!(*events.borrow(), vec![InputEvent::Submitted(expected)]);
    drop((keys, model));
    let view = area.view.downgrade();
    let buffer = area.view.buffer().downgrade();
    let retained = area.clone();
    drop(area);
    assert!(
        view.upgrade().is_some(),
        "A remaining host owner must retain its editor"
    );
    drop(retained);
    assert!(
        view.upgrade().is_none(),
        "Submit controller retained the closed TextArea"
    );
    assert!(
        buffer.upgrade().is_none(),
        "Closed TextArea retained its text buffer"
    );
    assert_eq!(
        Rc::strong_count(&events),
        1,
        "Closed TextArea retained its host callbacks"
    );
    println!("PASS: multiline Unicode change/submit callbacks, plain Return guard and final view/buffer/callback release");
}
