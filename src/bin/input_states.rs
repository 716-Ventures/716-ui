//! Painted Input endpoints; native focus/border presentation remains deliberate.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, TextField};
use std::time::Duration;

fn placeholder(parent: &gtk::Widget) -> Option<gtk::Widget> {
    if parent.css_name() == "placeholder" {
        return Some(parent.clone());
    }
    let mut child = parent.first_child();
    while let Some(node) = child {
        if let Some(found) = placeholder(&node) {
            return Some(found);
        }
        child = node.next_sibling();
    }
    None
}
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let directory = std::env::var("INPUT_STATE_OUTPUT").ok();
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.InputStates")
        .build();
    app.connect_activate(move |app| {
        let expected: serde_json::Value = serde_json::from_str(include_str!("../../reference/input-observation.json")).unwrap();
        let profile: serde_json::Value = serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
        assert_eq!(expected["profile_revision"],profile["revision"]);
        assert_eq!(expected["component_sha256"],profile["files"]["input.tsx"]);
        let colors=expected["backgrounds"][if dark{"dark"}else{"light"}].clone();
        let placeholder_rgb=expected["placeholder_rgb"][if dark{"dark"}else{"light"}].clone();
        let rgb=|value:&serde_json::Value| -> [u8;3] {std::array::from_fn(|i|value[i].as_u64().unwrap() as u8)};
        let bg=rgb(&colors["page"]);
        let display = gdk::Display::default().unwrap();
        seven_sixteen_ui::install_theme(
            &display,
            if dark { Appearance::Dark } else { Appearance::Light },
            1.0,
            true,
        );
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        row.add_css_class("seven-ui");
        // Keep automatic window focus away from the endpoint samples.
        let sink=gtk::Button::with_label("Focus sink");row.append(&sink);
        let mut samples = Vec::new();
        for state in ["default", "disabled", "focus", "placeholder", "placeholder-disabled"] {
            let field = TextField::new(state, "Placeholder", if state.starts_with("placeholder"){""}else{"Unicode λ"});
            field.entry.set_hexpand(false);
            field.entry.set_width_chars(0);
            field.entry.set_max_width_chars(0);
            field.entry.set_size_request(190, 36);
            field.entry.set_sensitive(!state.ends_with("disabled"));
            row.append(&field.widget);
            samples.push((state, field.entry));
        }
        let window = gtk::ApplicationWindow::builder().application(app)
            .title("716 UI Input states").child(&row).build();
        window.present();assert!(samples.iter().find(|(state,_)|*state=="focus").unwrap().1.grab_focus());
        let quit = app.clone();
        let directory = directory.clone();
        glib::timeout_add_local_once(Duration::from_millis(500), move || {
            let mut checks = Vec::new();
            let focused=samples.iter().find(|(state,_)|*state=="focus").unwrap().1.clone();
            for (state, entry) in samples {
                let bounds = entry.compute_bounds(&entry).unwrap();
                assert_eq!((bounds.width(), bounds.height()), (190.0, 36.0));
                let expected=rgb(&colors[if state.ends_with("disabled"){"disabled"}else{"default"}]);
                let placeholder_color = if state.starts_with("placeholder") {
                    let node=placeholder(entry.upcast_ref()).expect("Visible GTK placeholder node");
                    assert!(node.is_visible());
                    let color=node.color();let expected=rgb(&placeholder_rgb);
                    let actual=[color.red(),color.green(),color.blue()].map(|c|(c*255.).round() as u8);
                    assert_eq!(actual,expected,"{state}: placeholder color");
                    assert!((color.alpha()-1.).abs()<0.001,"{state}: placeholder alpha {}",color.alpha());
                    Some(actual)
                }else{None};
                let snapshot = gtk::Snapshot::new();
                snapshot.append_color(&gdk::RGBA::new(bg[0] as f32 / 255., bg[1] as f32 / 255., bg[2] as f32 / 255., 1.),
                    &gtk::graphene::Rect::new(0., 0., bounds.width(), bounds.height()));
                gtk::WidgetPaintable::new(Some(&entry)).snapshot(&snapshot, bounds.width() as f64, bounds.height() as f64);
                let texture = window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(), None);
                let stride = texture.width() as usize * 4;
                let mut pixels = vec![0; stride * texture.height() as usize];
                texture.download(&mut pixels, stride);
                let offset = texture.height() as usize / 2 * stride + (texture.width() as usize - 8) * 4;
                let actual = [pixels[offset + 2], pixels[offset + 1], pixels[offset]];
                if let Some(directory) = &directory {
                    std::fs::create_dir_all(directory).unwrap();
                    texture.save_to_png(format!("{directory}/{state}.png")).unwrap();
                }
                assert!(actual.iter().zip(expected).all(|(a, e)| a.abs_diff(e) <= 2), "{state}: painted {actual:?}, reference {expected:?}");
                checks.push(serde_json::json!({"state":state,"painted_rgb":actual,"expected_rgb":expected,"width":bounds.width(),"height":bounds.height(),"placeholder_rgb":placeholder_color}));
            }
            // Entry delegates focus to GtkText; this is real focus, not CSS flags.
            assert!(focused.state_flags().contains(gtk::StateFlags::FOCUS_WITHIN));
            let field=focused.compute_bounds(&row).unwrap();let bounds=row.compute_bounds(&row).unwrap();
            let snapshot=gtk::Snapshot::new();
            snapshot.append_color(&gdk::RGBA::new(bg[0] as f32/255.,bg[1] as f32/255.,bg[2] as f32/255.,1.),&bounds);
            gtk::WidgetPaintable::new(Some(&row)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
            let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),Some(&bounds));
            let stride=texture.width() as usize*4;let mut pixels=vec![0;stride*texture.height() as usize];texture.download(&mut pixels,stride);
            let x=(field.x()-3.) as usize;let y=(field.y()+field.height()/2.) as usize;let offset=y*stride+x*4;
            let actual=[pixels[offset+2],pixels[offset+1],pixels[offset]];let expected=rgb(&placeholder_rgb);
            if let Some(directory)=&directory {texture.save_to_png(format!("{directory}/actual-focus.png")).unwrap();}
            assert!(actual.iter().zip(expected).all(|(a,e)|a.abs_diff(e)<=2),"Actual Entry focus ring: painted {actual:?}, expected {expected:?}");
            checks.push(serde_json::json!({"state":"actual-focus-ring","painted_rgb":actual,"expected_rgb":expected}));
            println!("{}", serde_json::json!({"result":"pass","appearance":if dark{"dark"}else{"light"},"checks":checks,"scope":"desktop Input backgrounds, placeholder colors and actual native focus ring; full reference parity unqualified"}));
            quit.quit();
        });
    });
    app.run_with_args(&["716-ui-input-states"]);
}
