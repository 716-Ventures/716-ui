//! Rendered endpoint regression against the pinned Zinc button state contract.
//! State flags exercise GTK CSS; this does not qualify physical pointer delivery.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonVariant};
use std::time::Duration;

fn mix(fg: [u8; 3], bg: [u8; 3], alpha: f64) -> [u8; 3] {
    std::array::from_fn(|i| (fg[i] as f64 * alpha + bg[i] as f64 * (1.0 - alpha)).round() as u8)
}
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let directory = std::env::var("BUTTON_STATE_OUTPUT").ok();
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.ButtonStates")
        .build();
    app.connect_activate(move|app|{
        let bg=if dark{[9,9,11]}else{[255,255,255]};
        let primary=if dark{[250,250,250]}else{[24,24,27]};
        let secondary=if dark{[39,39,42]}else{[244,244,245]};
        let display=gdk::Display::default().unwrap();
        seven_sixteen_ui::install_theme(&display,if dark{Appearance::Dark}else{Appearance::Light},1.0,true);
        let panel=gtk::Box::new(gtk::Orientation::Vertical,12);panel.add_css_class("seven-ui");
        for margin in [gtk::PositionType::Top,gtk::PositionType::Bottom,gtk::PositionType::Left,gtk::PositionType::Right] {
            match margin {gtk::PositionType::Top=>panel.set_margin_top(12),gtk::PositionType::Bottom=>panel.set_margin_bottom(12),gtk::PositionType::Left=>panel.set_margin_start(12),_=>panel.set_margin_end(12)}
        }
        let mut samples=Vec::new();
        for (name,variant) in [("primary",ButtonVariant::Primary),("secondary",ButtonVariant::Secondary),("outline",ButtonVariant::Outline),("ghost",ButtonVariant::Ghost),("destructive",ButtonVariant::Destructive)] {
            let row=gtk::Box::new(gtk::Orientation::Horizontal,12);
            for state in ["default","hover","pressed","disabled","focus"] {
                let button=seven_sixteen_ui::button(&format!("{name} {state}"),variant,state=="disabled");
                button.set_size_request(190,40);
                if state=="hover" || state=="pressed" {button.set_state_flags(gtk::StateFlags::PRELIGHT,false);}
                if state=="pressed" {button.set_state_flags(gtk::StateFlags::ACTIVE,false);}
                if state=="focus" {button.set_state_flags(gtk::StateFlags::FOCUSED|gtk::StateFlags::FOCUS_VISIBLE,false);}
                row.append(&button);samples.push((name,state,button));
            }
            panel.append(&row);
        }
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI button states").child(&panel).build();
        window.present();let quit=app.clone();let directory=directory.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move||{
            let mut report=Vec::new();
            for (name,state,button) in samples {
                let hover=state=="hover" || state=="pressed";
                let base=match name {"primary"=>primary,"secondary"=>secondary,"destructive"=>[185,28,28],"outline" if dark=>mix(secondary,bg,0.3),_=>bg};
                let expected=if hover {match name {"primary"=>mix(primary,bg,0.9),"secondary"=>mix(secondary,bg,0.8),"destructive"=>mix([185,28,28],bg,0.9),_=>if dark{mix(secondary,bg,0.5)}else{secondary}}}else if state=="disabled"{mix(base,bg,0.5)}else{base};
                let bounds=button.compute_bounds(&button).unwrap();
                assert_eq!((bounds.width(),bounds.height()),(190.0,40.0));
                let snapshot=gtk::Snapshot::new();
                snapshot.append_color(&gdk::RGBA::new(bg[0] as f32/255.0,bg[1] as f32/255.0,bg[2] as f32/255.0,1.0),&gtk::graphene::Rect::new(0.0,0.0,bounds.width(),bounds.height()));
                gtk::WidgetPaintable::new(Some(&button)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
                let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),None);
                let stride=texture.width() as usize*4;let mut pixels=vec![0;stride*texture.height() as usize];texture.download(&mut pixels,stride);
                let offset=texture.height() as usize/2*stride+8*4;
                // GDK download uses Cairo ARGB32 (BGRA on this ARM64 little-endian profile).
                let actual=[pixels[offset+2],pixels[offset+1],pixels[offset]];
                assert!(actual.iter().zip(expected).all(|(a,e)|a.abs_diff(e)<=2),"{name}/{state}: painted {actual:?}, expected {expected:?}");
                assert_eq!(pixels[offset+3],255);
                report.push(serde_json::json!({"variant":name,"state":state,"painted_rgb":actual,"expected_rgb":expected,"border_box_width":bounds.width(),"border_box_height":bounds.height()}));
                if let Some(directory)=&directory {std::fs::create_dir_all(directory).unwrap();texture.save_to_png(format!("{directory}/{name}-{state}.png")).unwrap();}
            }
            println!("{}",serde_json::json!({"result":"pass","appearance":if dark{"dark"}else{"light"},"checks":report,"scope":"rendered state endpoints; pointer delivery, motion and full reference parity unqualified"}));
            quit.quit();
        });
    });
    app.run_with_args(&["716-ui-button-states"]);
}
