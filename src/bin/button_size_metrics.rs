//! Actual mapped geometry against the measured pinned browser size matrix.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonSize, ButtonVariant};
use std::time::Duration;

fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let scale = if std::env::args().any(|arg| arg == "--scale-2") {
        2.0
    } else {
        1.0
    };
    let capture = std::env::var("BUTTON_SIZE_CAPTURE").ok();
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.ButtonSizes")
        .build();
    app.connect_activate(move |app| {
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(), if dark {Appearance::Dark} else {Appearance::Light}, scale, true);
        let panel = gtk::Box::new(gtk::Orientation::Vertical, 12);
        panel.add_css_class("seven-ui");
        panel.set_margin_top(12);panel.set_margin_bottom(12);panel.set_margin_start(12);panel.set_margin_end(12);
        let mut samples = Vec::new();
        for (size_name,size) in [("xs",ButtonSize::ExtraSmall),("sm",ButtonSize::Small),("default",ButtonSize::Default),("lg",ButtonSize::Large)] {
            let row = gtk::Box::new(gtk::Orientation::Horizontal,12);
            for (name,variant) in [("default",ButtonVariant::Primary),("secondary",ButtonVariant::Secondary),("outline",ButtonVariant::Outline),("ghost",ButtonVariant::Ghost),("destructive",ButtonVariant::Destructive)] {
                let button = seven_sixteen_ui::button_sized(name,variant,size,false);
                button.set_valign(gtk::Align::Center);
                row.append(&button);samples.push((size_name,name,button));
            }
            panel.append(&row);
        }
        let disabled = seven_sixteen_ui::button_sized("Unavailable λ 日本語",ButtonVariant::Primary,ButtonSize::ExtraSmall,true);
        disabled.set_halign(gtk::Align::Start);panel.append(&disabled);
        let window = gtk::ApplicationWindow::builder().application(app).title("716 UI text-only button sizes").child(&panel).build();
        window.present();let quit=app.clone();let capture=capture.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move || {
            let reference:serde_json::Value=serde_json::from_str(include_str!("../../reference/button-size-metrics.json")).unwrap();
            let profile:serde_json::Value=serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
            assert_eq!(reference["profile_revision"],profile["revision"]);
            assert_eq!(reference["metrics"].as_array().unwrap().len(),20);
            let mut report=Vec::new();
            for (size,name,button) in samples {
                let expected=reference["metrics"].as_array().unwrap().iter().find(|value|value["size"]==size&&value["variant"]==name).unwrap();
                let bounds=button.compute_bounds(&button).unwrap();
                // Native text scaling preserves the one-pixel outline border.
                let border=if name=="outline" {2.0} else {0.0};
                let width=(expected["width"].as_f64().unwrap()-border)*scale+border;
                let tolerance=reference["native_width_tolerance_px"].as_f64().unwrap()*scale;
                assert!((bounds.width() as f64-width).abs()<=tolerance,"{size}/{name} scale {scale}: width {}, expected {width}",bounds.width());
                assert_eq!(bounds.height() as f64,expected["height"].as_f64().unwrap()*scale,"{size}/{name} height at scale {scale}");
                let label=button.child().unwrap().downcast::<gtk::Label>().unwrap();
                let font=label.pango_context().font_description().unwrap();
                assert!(font.family().unwrap().contains("Inter"),"Fallback font: {font}");
                assert_eq!(font.weight(),gtk::pango::Weight::Medium);
                assert_eq!(font.size() as f64/gtk::pango::SCALE as f64,expected["font_size_px"].as_f64().unwrap()*scale);
                assert_eq!(button.accessible_role(),gtk::AccessibleRole::Button);
                report.push(serde_json::json!({"size":size,"variant":name,"width":bounds.width(),"height":bounds.height(),"font":font.to_string()}));
            }
            assert!(!disabled.is_sensitive());assert_eq!(disabled.label().unwrap(),"Unavailable λ 日本語");
            if let Some(path)=capture {
                let bounds=panel.compute_bounds(&panel).unwrap();
                let snapshot=gtk::Snapshot::new();
                gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
                let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),Some(&bounds));
                texture.save_to_png(path).unwrap();
            }
            println!("{}",serde_json::json!({"result":"pass","appearance":if dark {"dark"} else {"light"},"text_scale":scale,"metrics":report,"scope":"text-only mapped geometry and native font; full appearance, physical input and motion parity unqualified"}));
            quit.quit();
        });
    });
    app.run_with_args(&["716-ui-button-sizes"]);
}
