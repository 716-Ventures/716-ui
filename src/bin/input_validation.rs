//! Feedback changes preserve actual focused editors, selection and host events.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, DocumentEditor, TextField};
use std::{cell::Cell, rc::Rc, time::Duration};

fn main() {
    let dark=std::env::args().any(|arg|arg=="--dark");
    let scale=if std::env::args().any(|arg|arg=="--scale-2"){2.0}else{1.0};
    let capture=std::env::var("INPUT_VALIDATION_CAPTURE").unwrap();
    let app=gtk::Application::builder().application_id("com.sevensixteen.InputValidation").build();
    app.connect_activate(move |app| {
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark{Appearance::Dark}else{Appearance::Light},scale,true);
        let panel=seven_sixteen_ui::column(12);panel.add_css_class("seven-ui");
        panel.set_margin_start(12);panel.set_margin_end(12);panel.set_margin_top(12);panel.set_margin_bottom(12);
        let field=TextField::new("Workspace","Workspace name","Draft λ 日本語");
        let editor=DocumentEditor::new("Document","Document λ 日本語").unwrap();
        panel.append(&field.widget);panel.append(&editor.widget);
        let choice=seven_sixteen_ui::Choice::new("Validation choice", &["First".into(),"Second".into()], "Second").unwrap();
        let toggle=seven_sixteen_ui::Toggle::new("Validation toggle",dark).unwrap();
        panel.append(&choice.widget);panel.append(&toggle.widget);
        let changes=Rc::new(Cell::new(0));let changed=changes.clone();field.on_event(move |_|changed.set(changed.get()+1));
        let changed=changes.clone();choice.on_changed(move |_|changed.set(changed.get()+1));
        let changed=changes.clone();toggle.on_changed(move |_|changed.set(changed.get()+1));
        for feedback in [&choice.validation,&toggle.validation] {
            feedback.set_message(Some("Choose a value that meets this form’s requirements.")).unwrap();
            assert!(feedback.set_message(Some(" ")).is_err());
            assert_eq!(feedback.label.text(),"Choose a value that meets this form’s requirements.");
        }
        let window=gtk::ApplicationWindow::builder().application(app).default_width(460).child(&panel).build();window.present();
        assert!(field.entry.grab_focus());field.entry.select_region(0,5);
        let focus=gtk::prelude::RootExt::focus(&window);
        let selection=field.entry.selection_bounds();
        let message="Choose a saved workspace. Literal <b>日本語</b> & λ";
        field.set_validation_message(Some(message)).unwrap();editor.set_validation_message(Some("Save a readable document name.")).unwrap();
        for invalid in ["", "bad\0message", &"λ".repeat(513)] {
            assert!(field.set_validation_message(Some(invalid)).is_err());
            assert_eq!(field.validation.label.text(),message);
        }
        field.set_validation_message(Some(message)).unwrap();
        field.set_validation_message(None).unwrap();assert!(!field.validation.label.is_visible());assert!(!field.entry.has_css_class("seven-invalid"));
        let long="Choose a saved workspace 日本語 λ. ".repeat(16);field.set_validation_message(Some(&long)).unwrap();
        let quit=app.clone();let capture=capture.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move || {
            assert_eq!(field.entry.text(),"Draft λ 日本語");assert_eq!(editor.area.value(),"Document λ 日本語");assert_eq!(changes.get(),0);
            assert!(field.entry.has_focus()||field.entry.focus_child().is_some());assert_eq!(field.entry.selection_bounds(),selection);
            assert_eq!(gtk::prelude::RootExt::focus(&window),focus);
            assert!(field.validation.label.is_visible()&&field.entry.has_css_class("seven-invalid"));
            let color=field.validation.label.color();let expected=if dark{[252,165,165]}else{[185,28,28]};
            for (actual,expected) in [color.red(),color.green(),color.blue()].iter().zip(expected){assert!((*actual*255.-expected as f32).abs()<1.);}
            let error=field.validation.label.compute_bounds(&panel).unwrap();let box_bounds=panel.compute_bounds(&panel).unwrap();
            assert!(error.x()>=0.&&error.x()+error.width()<=box_bounds.width()+0.5);assert!(error.height()>14.*scale as f32,"Long validation text did not wrap");
            let snapshot=gtk::Snapshot::new();gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot,box_bounds.width() as f64,box_bounds.height() as f64);
            window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),Some(&box_bounds)).save_to_png(capture).unwrap();
            // Each native editor owns PRIMARY in turn; simultaneous selections
            // in unrelated widgets compete for that shared system resource.
            assert!(editor.area.view.grab_focus());
            editor.area.view.buffer().select_range(&editor.area.view.buffer().iter_at_offset(3),&editor.area.view.buffer().start_iter());
            editor.set_validation_message(None).unwrap();editor.set_validation_message(Some("Save a readable document name.")).unwrap();
            glib::timeout_add_local_once(Duration::from_millis(200),move || {
                // Headless Weston has no active physical seat. Assert GTK's
                // exact retained logical focus, as the mapped button test does.
                assert_eq!(gtk::prelude::RootExt::focus(&window),Some(editor.area.view.clone().upcast()));
                assert_eq!(editor.area.view.buffer().selection_bounds().map(|(a,b)|(a.offset(),b.offset())),Some((0,3)));
                assert_eq!(editor.area.value(),"Document λ 日本語");assert_eq!(field.entry.text(),"Draft λ 日本語");assert_eq!(changes.get(),0);
                field.validation.label.select_region(0,6);let selection=field.validation.label.selection_bounds();
                field.set_validation_message(Some(&long)).unwrap();assert_eq!(field.validation.label.selection_bounds(),selection);
                editor.set_validation_message(None).unwrap();assert!(!editor.area.view.has_css_class("seven-invalid"));
                assert!(choice.control.grab_focus());
                let focus=gtk::prelude::RootExt::focus(&window);
                choice.set_validation_message(None).unwrap();choice.set_validation_message(Some("Choose a permitted option.")).unwrap();
                assert_eq!(gtk::prelude::RootExt::focus(&window),focus);
                assert_eq!(choice.value().as_deref(),Some("Second"));
                assert!(choice.set_label("Renamed choice 日本語").unwrap());
                assert_eq!(choice.heading.text(),"Renamed choice 日本語");
                assert_eq!(gtk::prelude::RootExt::focus(&window),focus);
                for invalid in ["", "bad\0label", &"λ".repeat(129)] {
                    assert!(choice.set_label(invalid).is_err());
                    assert_eq!(choice.heading.text(),"Renamed choice 日本語");
                }
                choice.heading.select_region(0,7);let selected_heading=choice.heading.selection_bounds();
                assert!(!choice.set_label("Deferred choice").unwrap());
                assert_eq!(choice.heading.text(),"Renamed choice 日本語");assert_eq!(choice.heading.selection_bounds(),selected_heading);
                choice.heading.select_region(0,0);assert!(choice.control.grab_focus());
                assert!(choice.set_label("Validation choice").unwrap());assert_eq!(choice.value().as_deref(),Some("Second"));
                assert_eq!(changes.get(),0);
                toggle.control.set_inconsistent(dark);
                assert!(toggle.control.grab_focus());
                let focus=gtk::prelude::RootExt::focus(&window);
                toggle.set_validation_message(None).unwrap();toggle.set_validation_message(Some("Confirm this form requirement.")).unwrap();
                assert_eq!(gtk::prelude::RootExt::focus(&window),focus);assert_eq!(toggle.control.is_active(),dark);assert_eq!(toggle.control.is_inconsistent(),dark);
                assert_eq!(changes.get(),0);
                choice.validation.label.select_region(0,6);let selection=choice.validation.label.selection_bounds();
                choice.set_validation_message(Some("Choose a permitted option.")).unwrap();assert_eq!(choice.validation.label.selection_bounds(),selection);
                choice.set_validation_message(None).unwrap();toggle.set_validation_message(None).unwrap();
                assert!(!choice.control.has_css_class("seven-invalid")&&!toggle.control.has_css_class("seven-invalid"));
                println!("PASS: Choice label changes preserve focus, value and event ownership; invalid labels are atomic and selected headings defer replacement");
                println!("PASS: choice/toggle feedback preserves focused values, boolean state and event ownership; identical captions retain selection and clearing removes invalid styling");
                println!("PASS: bounded literal validation feedback preserves focused text, selection and host events; invalid messages are atomic, repeated feedback retains selection, clear recovers state, long errors wrap");quit.quit();
            });
        });
    });
    app.run_with_args(&["716-input-validation"]);
}
