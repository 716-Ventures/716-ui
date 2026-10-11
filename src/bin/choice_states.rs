//! Real GTK Choice trigger paint/geometry; no popup or physical input claim.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, Choice};
use std::time::Duration;
fn main() {
    let dark=std::env::args().any(|a|a=="--dark");
    let output=std::env::var("CHOICE_STATE_OUTPUT").unwrap();
    let app=gtk::Application::builder().application_id("com.sevensixteen.ChoiceStates").build();
    app.connect_activate(move |app| {
        let reference:serde_json::Value=serde_json::from_str(include_str!("../../reference/select-observation.json")).unwrap();
        let profile:serde_json::Value=serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
        assert_eq!(reference["component_sha256"],profile["files"]["select.tsx"]);
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark {Appearance::Dark}else{Appearance::Light},1.,true);
        let panel=gtk::Box::new(gtk::Orientation::Vertical,12);panel.add_css_class("seven-ui");
        panel.set_margin_top(20);panel.set_margin_bottom(20);panel.set_margin_start(20);panel.set_margin_end(20);
        let sink=gtk::Button::with_label("Focus sink");panel.append(&sink);
        let mut controls=Vec::new();
        for (name,hover,disabled) in [("Normal",false,false),("Hover",true,false),("Disabled",false,true),("Focused",false,false)] {
            let choice=Choice::new(name,&["Alpha λ".into(),"Beta 日本語".into()],"Alpha λ").unwrap();
            choice.control.set_size_request(190,-1);choice.control.set_halign(gtk::Align::Start);
            choice.control.set_sensitive(!disabled);
            let button=choice.control.first_child().expect("DropDown trigger");assert_eq!(button.css_name(),"button");
            if hover { choice.control.set_state_flags(gtk::StateFlags::PRELIGHT,false);button.set_state_flags(gtk::StateFlags::PRELIGHT,false); }
            choice.widget.set_margin_start(8);choice.widget.set_margin_end(8);choice.widget.set_margin_bottom(8);
            panel.append(&choice.widget);controls.push((choice,button,name,hover,disabled));
        }
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI Choice trigger states").child(&panel).build();window.present();sink.grab_focus();
        assert!(controls.last().unwrap().0.control.grab_focus(),"Actual Choice focus");
        let output=output.clone();let quit=app.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move || {
            let snapshot=gtk::Snapshot::new();let bounds=panel.compute_bounds(&panel).unwrap();
            gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
            let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),None);
            std::fs::create_dir_all(&output).unwrap();texture.save_to_png(format!("{output}/choices.png")).unwrap();
            let stride=texture.width() as usize*4;let mut pixels=vec![0;stride*texture.height() as usize];texture.download(&mut pixels,stride);
            let mut failures=Vec::new();
            for (choice,button,name,hover,disabled) in controls {
                let r=choice.control.compute_bounds(&panel).unwrap();let i=(r.y()+r.height()/2.).round() as usize*stride+(r.x()+114.).round() as usize*4;
                let painted=[pixels[i+2],pixels[i+1],pixels[i]];
                let bg=if dark {if hover {[24,24,27]}else{[18,18,20]}}else{[255,255,255]};
                let page=if dark {[9,9,11]}else{[255,255,255]};
                let expected=bg.into_iter().zip(page).map(|(a,b)|if disabled {((a as f64+b as f64)/2.).round() as u8}else{a}).collect::<Vec<_>>();
                println!("{}",serde_json::json!({"dark":dark,"state":name,"width":r.width(),"height":r.height(),"painted_rgb":painted,"expected_rgb":expected,"font":button.pango_context().font_description().map(|d|d.to_string()),"value":choice.value()}));
                if (r.width(),r.height())!=(190.,36.) {failures.push(format!("{name}: trigger geometry"));}
                if !painted.into_iter().zip(expected).all(|(a,b)|a.abs_diff(b)<=2) {failures.push(format!("{name}: trigger background"));}
                if name=="Focused" {
                    let expected=if dark {[161,161,170]}else{[113,113,122]};
                    for (edge,x,y) in [("top",r.x()+100.,r.y()-3.),("bottom",r.x()+100.,r.y()+r.height()+3.),("left",r.x()-3.,r.y()+18.),("right",r.x()+r.width()+3.,r.y()+18.)] {
                        let i=y.round() as usize*stride+x.round() as usize*4;
                        let ring=[pixels[i+2],pixels[i+1],pixels[i]];
                        println!("{}",serde_json::json!({"dark":dark,"edge":edge,"actual_focus_ring_rgb":ring,"expected_rgb":expected}));
                        if !ring.into_iter().zip(expected).all(|(a,b)|a.abs_diff(b)<=2) {failures.push(format!("Focused: actual trigger ring {edge}"));}
                    }
                }
                if choice.value().as_deref()!=Some("Alpha λ") {failures.push(format!("{name}: unexpected value"));}
                if button.pango_context().font_description().is_none_or(|d|d.weight()!=gtk::pango::Weight::Normal) {failures.push(format!("{name}: trigger font weight"));}
            }
            assert!(failures.is_empty(),"Choice reference endpoints: {failures:?}");quit.quit();
        });
    });
    app.run_with_args::<&str>(&[]);
}
