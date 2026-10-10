//! Painted transition checks, including reversal and reduced-motion endpoints.
//! Logical timer samples do not qualify physical presentation or browser parity.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonVariant};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

fn paint(window: &gtk::ApplicationWindow, button: &gtk::Button, bg: [u8; 3]) -> [u8; 3] {
    let snapshot = gtk::Snapshot::new();
    snapshot.append_color(
        &gdk::RGBA::new(
            bg[0] as f32 / 255.,
            bg[1] as f32 / 255.,
            bg[2] as f32 / 255.,
            1.,
        ),
        &gtk::graphene::Rect::new(0., 0., button.width() as f32, button.height() as f32),
    );
    gtk::WidgetPaintable::new(Some(button)).snapshot(
        &snapshot,
        button.width() as f64,
        button.height() as f64,
    );
    let texture = window
        .renderer()
        .unwrap()
        .render_texture(&snapshot.to_node().unwrap(), None);
    let stride = texture.width() as usize * 4;
    let mut pixels = vec![0; stride * texture.height() as usize];
    texture.download(&mut pixels, stride);
    let p = texture.height() as usize / 2 * stride + 8 * 4;
    assert_eq!(pixels[p + 3], 255);
    [pixels[p + 2], pixels[p + 1], pixels[p]]
}
fn near(a: [u8; 3], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(a, b)| a.abs_diff(b) <= 2)
}
struct Motion {
    phase: u8,
    started: Instant,
    samples: Vec<serde_json::Value>,
    saw_intermediate: [bool; 2],
}
fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let reduced = std::env::args().any(|arg| arg == "--reduced-motion");
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.ButtonMotion")
        .build();
    app.connect_activate(move|app|{
        gtk::Settings::default().unwrap().set_property("gtk-enable-animations",true);
        let bg=if dark{[9,9,11]}else{[255,255,255]};
        let primary=if dark{[250,250,250]}else{[24,24,27]};
        let accent=if dark{[39,39,42]}else{[244,244,245]};
        let mix=|fg:[u8;3],alpha:f64|std::array::from_fn(|i|(fg[i] as f64*alpha+bg[i] as f64*(1.-alpha)).round() as u8);
        let endpoints=[(primary,mix(primary,0.9)),(bg,mix(accent,if dark{0.5}else{1.}))];
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark{Appearance::Dark}else{Appearance::Light},1.,reduced);
        let panel=gtk::Box::new(gtk::Orientation::Vertical,12);panel.add_css_class("seven-ui");
        let buttons=[seven_sixteen_ui::button("Primary",ButtonVariant::Primary,false),seven_sixteen_ui::button("Ghost",ButtonVariant::Ghost,false)];
        for button in &buttons{button.set_size_request(190,40);panel.append(button);}
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI button motion").child(&panel).build();window.present();
        let quit=app.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move||{
            for (button,(normal,_)) in buttons.iter().zip(endpoints){assert!(near(paint(&window,button,bg),normal));button.set_state_flags(gtk::StateFlags::PRELIGHT,false);}
            let state=Rc::new(RefCell::new(Motion {phase:0,started:Instant::now(),samples:Vec::new(),saw_intermediate:[false;2]}));
            glib::timeout_add_local(Duration::from_millis(10),move||{
                let mut state=state.borrow_mut();let phase=state.phase;let elapsed=state.started.elapsed().as_millis();
                let colors=buttons.iter().map(|button|paint(&window,button,bg)).collect::<Vec<_>>();
                for (i,actual) in colors.iter().enumerate(){
                    let (normal,hover)=endpoints[i];
                    assert!(actual.iter().enumerate().all(|(c,v)|*v>=normal[c].min(hover[c]).saturating_sub(2)&&*v<=normal[c].max(hover[c]).saturating_add(2)),"Unexpected transition excursion: {actual:?}, {normal:?} -> {hover:?}");
                    if phase==0 && !near(*actual,normal)&&!near(*actual,hover){state.saw_intermediate[i]=true;}
                    if reduced && elapsed>=60{let expected=if phase==0||phase==2{hover}else{normal};assert!(near(*actual,expected),"Reduced motion was still animating: {actual:?}, expected {expected:?}");}
                }
                state.samples.push(serde_json::json!({"phase":phase,"elapsed_ms":elapsed,"primary":colors[0],"ghost":colors[1]}));
                let duration=if phase==2{60}else{240};
                if elapsed<duration{return glib::ControlFlow::Continue}
                if phase!=2{for (actual,(normal,hover)) in colors.iter().zip(endpoints){assert!(near(*actual,if phase==0{hover}else{normal}));}}
                if phase==3{
                    if !reduced{assert!(state.saw_intermediate.iter().all(|value| *value),"Transition skipped intermediate painted values: {:?}",state.saw_intermediate);}
                    println!("{}",serde_json::json!({"result":"pass","dark":dark,"reduced_motion":reduced,"samples":state.samples,"scope":"painted endpoints, bounded excursions, reversal/interruption and reduced motion; no physical timing or full browser curve parity claim"}));quit.quit();return glib::ControlFlow::Break
                }
                state.phase+=1;state.started=Instant::now();
                for button in &buttons{if state.phase==2{button.set_state_flags(gtk::StateFlags::PRELIGHT,false);}else{button.unset_state_flags(gtk::StateFlags::PRELIGHT);}}
                glib::ControlFlow::Continue
            });
        });
    });
    app.run_with_args(&["716-ui-button-motion"]);
}
