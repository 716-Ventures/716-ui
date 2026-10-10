//! Compare actual default horizontal Notebook paint/geometry with pinned Tabs.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, Tabs};
use std::time::Duration;

fn find(parent: &gtk::Widget, name: &str) -> Option<gtk::Widget> {
    if parent.css_name() == name { return Some(parent.clone()); }
    let mut child = parent.first_child();
    while let Some(node) = child {
        if let Some(found) = find(&node, name) { return Some(found); }
        child = node.next_sibling();
    }
    None
}
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let output = std::env::var("TABS_STATE_OUTPUT").unwrap();
    let app = gtk::Application::builder().application_id("com.sevensixteen.TabsStates").build();
    app.connect_activate(move |app| {
        let expected: serde_json::Value = serde_json::from_str(include_str!("../../reference/tabs-observation.json")).unwrap();
        let profile: serde_json::Value = serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
        assert_eq!(expected["component_sha256"], profile["files"]["tabs.tsx"]);
        assert_eq!(expected["profile_revision"], profile["revision"]);
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(), if dark { Appearance::Dark } else { Appearance::Light }, 1., true);
        let tabs = Tabs::new("Reference settings").unwrap();
        let account = gtk::Label::new(Some("Account λ content"));
        let security = gtk::Label::new(Some("Security 日本語 content"));
        tabs.set_pages(&[("account".into(), "Account".into(), account.clone().upcast()), ("security".into(), "Security".into(), security.clone().upcast())]).unwrap();
        let panel = gtk::Box::new(gtk::Orientation::Vertical, 12); panel.add_css_class("seven-ui");
        let sink = gtk::Button::with_label("Focus sink"); panel.append(&sink); panel.append(&tabs.widget);
        let window = gtk::ApplicationWindow::builder().application(app).title("716 UI Tabs states").child(&panel).build();
        window.present(); sink.grab_focus();
        let quit = app.clone(); let output = output.clone();
        glib::timeout_add_local_once(Duration::from_millis(500), move || {
            let header = find(tabs.widget.upcast_ref(), "header").expect("Notebook header");
            let strip=header.compute_bounds(&panel).unwrap(); let notebook=tabs.widget.compute_bounds(&panel).unwrap();
            assert_eq!((strip.x(),strip.y(),strip.width()),(notebook.x(),notebook.y(),notebook.width()),"Inherited Notebook outer border");
            let label = tabs.widget.tab_label(&account).unwrap();
            let trigger = label.parent().expect("Tab label parent");
            let snapshot = gtk::Snapshot::new();
            let bounds = panel.compute_bounds(&panel).unwrap();
            gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot, bounds.width() as f64, bounds.height() as f64);
            let texture = window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(), None);
            std::fs::create_dir_all(&output).unwrap(); texture.save_to_png(format!("{output}/tabs.png")).unwrap();
            let stride = texture.width() as usize * 4; let mut pixels = vec![0; stride * texture.height() as usize]; texture.download(&mut pixels, stride);
            let sample = |widget: &gtk::Widget| {
                let r = widget.compute_bounds(&panel).unwrap();
                let x = (r.x() + if widget.css_name()=="header" { r.width()-4. } else { 3. }).round() as usize; let y = (r.y() + r.height()/2.).round() as usize;
                let i = y * stride + x * 4;
                serde_json::json!({"measure":widget.measure(gtk::Orientation::Vertical,-1).0,"css_name":widget.css_name().to_string(),"height":r.height(),"width":r.width(),"painted_rgb":[pixels[i+2],pixels[i+1],pixels[i]]})
            };
            let inactive_label=tabs.widget.tab_label(&security).unwrap();
            let inactive=inactive_label.parent().unwrap();
            let observed = serde_json::json!({"header":sample(&header),"active":sample(&trigger),"inactive":sample(&inactive),"font":label.pango_context().font_description().unwrap().to_string()});
            println!("{observed}");
            let theme = if dark { "dark" } else { "light" };
            for (name, metric, color) in [("header", "list_height", "list_rgb"), ("active", "trigger_height", "active_rgb"), ("inactive", "trigger_height", "list_rgb")] {
                let height=observed[name]["height"].as_f64().unwrap();
                assert!((height-expected["metrics"][metric].as_f64().unwrap()).abs()<=1., "{name}: measured height {height}");
                for channel in 0..3 {
                    let a=observed[name]["painted_rgb"][channel].as_i64().unwrap(); let b=expected[color][theme][channel].as_i64().unwrap();
                    assert!((a-b).abs()<=2,"{name}: paint {a}, reference {b}");
                }
            }
            // Native text shaping/rounding spans 73–75 px on qualified Ubuntu/Debian
            // Inter builds; the browser measures 74.047 px. Heights/paint stay strict.
            assert!((observed["active"]["width"].as_f64().unwrap()-expected["metrics"]["account_trigger_width"].as_f64().unwrap()).abs()<=2., "Natural Account trigger width");
            for (node, active) in [(&label,true),(&inactive_label,false)] {
                let font=node.pango_context().font_description().unwrap();
                assert!(font.family().unwrap().contains("Inter"),"Unqualified fallback font: {font}");
                assert_eq!(font.weight(),gtk::pango::Weight::Medium);
                assert_eq!(font.size(),14*gtk::pango::SCALE);
                let color=node.color();
                let rgb=[color.red(),color.green(),color.blue()].map(|c|(c*255.).round() as i32);
                let wanted=if active {if dark {[250,250,250]}else{[9,9,11]}} else if dark {[161,161,170]}else{[9,9,11]};
                assert_eq!(rgb,wanted,"Tab label color");
                let alpha=if active || dark {1.}else{0.6};
                assert!((color.alpha()-alpha).abs()<0.001,"Tab label opacity");
            }
            assert_eq!(tabs.selected_key().as_deref(),Some("account"));
            assert!(tabs.select_key("security")); assert_eq!(tabs.selected_key().as_deref(),Some("security"));
            quit.quit();
        });
    });
    app.run_with_args::<&str>(&[]);
}
