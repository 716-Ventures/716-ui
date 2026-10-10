//! Actual native checkbox geometry, state paint and disabled semantics.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, Toggle};
use std::time::Duration;
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let output = std::env::var("TOGGLE_STATE_OUTPUT").unwrap();
    let app = gtk::Application::builder().application_id("com.sevensixteen.ToggleStates").build();
    app.connect_activate(move |app| {
        let reference: serde_json::Value = serde_json::from_str(include_str!("../../reference/checkbox-observation.json")).unwrap();
        let profile: serde_json::Value = serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
        assert_eq!(reference["component_sha256"],profile["files"]["checkbox.tsx"]);
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark {Appearance::Dark}else{Appearance::Light},1.,true);
        let panel=gtk::Box::new(gtk::Orientation::Vertical,12);panel.add_css_class("seven-ui");
        panel.set_margin_top(20);panel.set_margin_bottom(20);panel.set_margin_start(20);panel.set_margin_end(20);
        let sink=gtk::Button::with_label("Focus sink");panel.append(&sink);
        let mut controls=Vec::new();
        for (label,active,disabled) in [("Unchecked λ 日本語",false,false),("Checked",true,false),("Disabled unchecked",false,true),("Disabled checked",true,true)] {
            let toggle=Toggle::new(label,active).unwrap();toggle.control.set_sensitive(!disabled);
            panel.append(&toggle.widget);controls.push((toggle,active,disabled));
        }
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI Checkbox states").child(&panel).build();window.present();sink.grab_focus();
        let quit=app.clone();let output=output.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move || {
            let snapshot=gtk::Snapshot::new();let bounds=panel.compute_bounds(&panel).unwrap();
            gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
            let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),None);
            std::fs::create_dir_all(&output).unwrap();texture.save_to_png(format!("{output}/toggles.png")).unwrap();
            let stride=texture.width() as usize*4;let mut pixels=vec![0;stride*texture.height() as usize];texture.download(&mut pixels,stride);
            for (toggle,active,disabled) in controls {
                let check=toggle.control.first_child().expect("CheckButton indicator");assert_eq!(check.css_name(),"check");
                let r=check.compute_bounds(&panel).unwrap();let i=(r.y()+3.).round() as usize*stride+(r.x()+3.).round() as usize*4;
                let painted=[pixels[i+2],pixels[i+1],pixels[i]];
                let background=if active {if dark {[250,250,250]}else{[24,24,27]}}else if dark {[18,18,20]}else{[255,255,255]};
                let page=if dark {[9,9,11]}else{[255,255,255]};
                let expected=background.map(|c|c as f64).into_iter().zip(page).map(|(a,b)|if disabled {(a+b as f64)/2.}else{a}).map(|c|c.round() as u8).collect::<Vec<_>>();
                println!("{}",serde_json::json!({"dark":dark,"checked":active,"disabled":disabled,"width":r.width(),"height":r.height(),"painted_rgb":painted,"expected_rgb":expected}));
                assert_eq!((r.width(),r.height()),(16.,16.),"Checkbox reference border box");
                assert!(painted.into_iter().zip(expected).all(|(a,b)|a.abs_diff(b)<=2),"Checkbox state background");
                if disabled {toggle.control.activate_action("control.toggle",None).unwrap();assert_eq!(toggle.control.is_active(),active,"Disabled activation changed state");}
                else {toggle.control.activate_action("control.toggle",None).unwrap();assert_eq!(toggle.control.is_active(),!active,"Native activation did not toggle");}
            }
            quit.quit();
        });
    });app.run_with_args::<&str>(&[]);
}
