//! Actual mapped updates retain the button's focus and host action identity.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, ActionButton, Appearance, ButtonProps};
use std::{cell::Cell, rc::Rc, time::Duration};

fn props(phase: usize) -> ButtonProps {
    use serde_json::json;
    ButtonProps::from_value(match phase {
        0 => json!({"label":"default"}),
        1 => json!({"label":"default","size":"xs","icon":"plus"}),
        2 => json!({"label":"Add link","variant":"link","size":"xs","icon":"arrow-right","icon_position":"only"}),
        3 => json!({"label":"outline","variant":"outline","size":"lg","icon":"arrow-right","icon_position":"trailing"}),
        _ => json!({"label":"secondary","variant":"secondary","disabled":phase==5}),
    }).unwrap()
}
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let rtl = std::env::args().any(|arg| arg == "--rtl");
    let scale = if std::env::args().any(|arg| arg == "--scale-2") {
        2.0
    } else {
        1.0
    };
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.ButtonReconcile")
        .build();
    app.connect_activate(move |app| {
        gtk::Widget::set_default_direction(if rtl{gtk::TextDirection::Rtl}else{gtk::TextDirection::Ltr});
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark{Appearance::Dark}else{Appearance::Light},scale,true);
        let panel=gtk::Box::new(gtk::Orientation::Vertical,8);panel.add_css_class("seven-ui");
        let mut button=ActionButton::new(&props(0)).unwrap();
        button.widget.set_halign(gtk::Align::Start);button.widget.set_valign(gtk::Align::Center);button.widget.add_css_class("host-custom-class");
        panel.append(&button.widget);
        let count=Rc::new(Cell::new(0));let clicked=count.clone();button.widget.connect_clicked(move |_|clicked.set(clicked.get()+1));
        let original=button.widget.clone();
        let window=gtk::ApplicationWindow::builder().application(app).child(&panel).build();window.present();assert!(button.widget.grab_focus());
        let phase=Cell::new(0);let quit=app.clone();
        glib::timeout_add_local(Duration::from_millis(400),move || {
            let current=phase.get();let declaration=props(current);
            assert_eq!(button.widget,original);assert!(button.widget.has_css_class("host-custom-class"));assert_eq!(count.get(),0,"Reconciliation dispatched an action");
            if current<5 {
                assert_eq!(gtk::prelude::GtkWindowExt::focus(&window),Some(button.widget.clone().upcast()),"Button focus changed during reconciliation");
                let text:serde_json::Value=serde_json::from_str(include_str!("../../reference/button-size-metrics.json")).unwrap();
                let icons:serde_json::Value=serde_json::from_str(include_str!("../../reference/button-icon-metrics.json")).unwrap();
                let size=if current==1||current==2{"xs"}else if current==3{"lg"}else{"default"};
                let variant=if current==2{"link"}else if current==3{"outline"}else if current==4{"secondary"}else{"default"};
                let mode=if current==1{"leading"}else if current==2{"only"}else{"trailing"};
                let reference=if current==0||current==4{&text}else{&icons};
                let expected=reference["metrics"].as_array().unwrap().iter().find(|v|v["size"]==size&&v["variant"]==variant&& (current==0||current==4||(v["mode"]==mode&&v["icon"]==if current==1{"plus"}else{"arrow-right"}&&v["appearance"]==if dark{"dark"}else{"light"}))).unwrap();
                let bounds=button.widget.compute_bounds(&button.widget).unwrap();let border=if current==3{2.0}else{0.0};let width=(expected["width"].as_f64().unwrap()-border)*scale+border;
                assert!((bounds.width() as f64-width).abs()<=scale,"phase {current}: width {} expected {width}",bounds.width());assert_eq!(bounds.height() as f64,expected["height"].as_f64().unwrap()*scale);
            }
            let child=button.widget.child().unwrap();button.reconcile(&declaration).unwrap();assert_eq!(button.widget.child(),Some(child.clone()),"Identical content replaced children");
            assert_eq!(button.widget.direction(),if rtl{gtk::TextDirection::Rtl}else{gtk::TextDirection::Ltr});
            if current==1||current==3 {
                let first=child.first_child().unwrap().compute_bounds(&child).unwrap();let last=child.last_child().unwrap().compute_bounds(&child).unwrap();
                assert_eq!(first.x()>last.x(),rtl,"Retained icon content lost reading direction");
            }
            if current==5 {
                assert!(!button.widget.is_sensitive());
                let mut invalid=declaration.clone();invalid.label=" ".into();invalid.disabled=false;assert!(button.reconcile(&invalid).is_err());assert_eq!(button.widget.child(),Some(child));assert!(!button.widget.is_sensitive());
                button.reconcile(&props(4)).unwrap();button.widget.emit_clicked();assert_eq!(count.get(),1,"Host action owner was lost or duplicated");
                println!("PASS: mapped button updates retain identity, geometry, focus, host classes and action owner; invalid changes preserve content and disabled state");quit.quit();return glib::ControlFlow::Break;
            }
            button.reconcile(&props(current+1)).unwrap();phase.set(current+1);glib::ControlFlow::Continue
        });
    });
    app.run_with_args(&["716-button-reconcile"]);
}
