//! Natural Button metrics for comparison with the pinned browser component.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonVariant};
use std::time::Duration;
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.ButtonMetrics")
        .build();
    app.connect_activate(move|app|{
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark{Appearance::Dark}else{Appearance::Light},1.,true);
        let row=gtk::Box::new(gtk::Orientation::Horizontal,12);row.add_css_class("seven-ui");
        let mut samples=Vec::new();
        for (name,variant) in [("default",ButtonVariant::Primary),("secondary",ButtonVariant::Secondary),("outline",ButtonVariant::Outline),("ghost",ButtonVariant::Ghost),("destructive",ButtonVariant::Destructive)]{
            let button=seven_sixteen_ui::button(name,variant,false);row.append(&button);samples.push((name,button));
        }
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI natural Button metrics").child(&row).build();window.present();let quit=app.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move||{
            let expected:serde_json::Value=serde_json::from_str(include_str!("../../reference/button-metrics.json")).unwrap();
            let profile:serde_json::Value=serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
            assert_eq!(expected["profile_revision"],profile["revision"]);
            let metrics=samples.iter().map(|(name,button)|{
                let label=button.child().unwrap().downcast::<gtk::Label>().unwrap();
                let font=label.pango_context().font_description().unwrap();
                let (_,text)=label.layout().pixel_extents();
                // GTK width()/height() are content-box dimensions. Browser
                // getBoundingClientRect measures the border box instead.
                let bounds=button.compute_bounds(button).unwrap();
                let width=expected["widths"][*name].as_f64().unwrap();
                assert!((bounds.width() as f64-width).abs()<=expected["native_width_tolerance_px"].as_f64().unwrap(),"{name}: native border box {}, observed reference {width}",bounds.width());
                assert_eq!(bounds.height(),expected["border_box_height"].as_f64().unwrap() as f32);
                assert!(font.family().unwrap().contains("Inter"),"Unqualified fallback font: {font}");
                assert_eq!(font.weight(),gtk::pango::Weight::Medium);
                serde_json::json!({"variant":name,"width":bounds.width(),"height":bounds.height(),"content_width":button.width(),"content_height":button.height(),"font":font.to_string(),"text_width":text.width(),"text_height":text.height()})
            }).collect::<Vec<_>>();
            println!("{}",serde_json::json!({"appearance":if dark{"dark"}else{"light"},"metrics":metrics}));quit.quit();
        });
    });
    app.run_with_args(&["716-ui-button-metrics"]);
}
