//! Mapped native icon geometry against the independently measured browser.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonSize, ButtonVariant, Icon, IconPosition};
use std::time::Duration;
fn main() {
    let dark = std::env::args().any(|a| a == "--dark");
    let scale = if std::env::args().any(|a| a == "--scale-2") {
        2.0
    } else {
        1.0
    };
    let mode = std::env::var("BUTTON_ICON_MODE").unwrap();
    let capture = std::env::var("BUTTON_ICON_CAPTURE").unwrap();
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.IconMetrics")
        .build();
    app.connect_activate(move |app| {
  seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark{Appearance::Dark}else{Appearance::Light},scale,true);
  let panel=gtk::Box::new(gtk::Orientation::Vertical,8);panel.add_css_class("seven-ui");
  let mut samples=Vec::new();
  for (size_name,size) in [("xs",ButtonSize::ExtraSmall),("sm",ButtonSize::Small),("default",ButtonSize::Default),("lg",ButtonSize::Large)] {
   let variants=[("default",ButtonVariant::Primary),("secondary",ButtonVariant::Secondary),("outline",ButtonVariant::Outline),("ghost",ButtonVariant::Ghost),("destructive",ButtonVariant::Destructive),("link",ButtonVariant::Link)];
   for chunk in variants.chunks(3) {
    let row=gtk::Box::new(gtk::Orientation::Horizontal,8);
    for &(name,variant) in chunk {
     let icons=if mode=="only"{vec![("plus",Icon::Plus),("arrow-right",Icon::ArrowRight)]}else if mode=="leading"{vec![("plus",Icon::Plus)]}else{vec![("arrow-right",Icon::ArrowRight)]};
     for (icon_name,icon) in icons {
      let button=if mode=="only"{seven_sixteen_ui::icon_button(&format!("Add {name}"),icon,variant,size,false)}else{seven_sixteen_ui::button_with_icon(name,icon,if mode=="leading"{IconPosition::Leading}else{IconPosition::Trailing},variant,size,false)}.unwrap();
      button.set_valign(gtk::Align::Center);row.append(&button);samples.push((size_name,name,icon_name,button));
     }
    }
    panel.append(&row);
   }
  }
  assert!(seven_sixteen_ui::icon_button(" ",Icon::Plus,ButtonVariant::Primary,ButtonSize::Default,false).is_err());
  assert!(seven_sixteen_ui::icon_button("bad\nname",Icon::Plus,ButtonVariant::Primary,ButtonSize::Default,false).is_err());
  let disabled=seven_sixteen_ui::icon_button("Unavailable λ 日本語",Icon::Plus,ButtonVariant::Primary,ButtonSize::Default,true).unwrap();assert!(!disabled.is_sensitive());
  let window=gtk::ApplicationWindow::builder().application(app).title("716 UI icon button geometry").child(&panel).build();window.present();
  let quit=app.clone();let mode=mode.clone();let capture=capture.clone();
  glib::timeout_add_local_once(Duration::from_millis(500),move || {
   let reference:serde_json::Value=serde_json::from_str(include_str!("../../reference/button-icon-metrics.json")).unwrap();
   assert_eq!(reference["metrics"].as_array().unwrap().len(),192);
   let mut report=Vec::new();
   for (size,name,icon_name,button) in samples {
    let expected=reference["metrics"].as_array().unwrap().iter().find(|v|v["size"]==size&&v["variant"]==name&&v["mode"]==mode&&v["icon"]==icon_name&&v["appearance"]==if dark{"dark"}else{"light"}).unwrap();
    let button_bounds=button.compute_bounds(&button).unwrap();
    let border=if name=="outline"{2.0}else{0.0};
    let width=(expected["width"].as_f64().unwrap()-border)*scale+border;
    assert!((button_bounds.width() as f64-width).abs()<=scale,"{mode}/{size}/{name}/{icon_name} width {} expected {width}",button_bounds.width());
    assert_eq!(button_bounds.height() as f64,expected["height"].as_f64().unwrap()*scale,"{mode}/{size}/{name} height");
    assert_eq!(button.accessible_role(),gtk::AccessibleRole::Button);
    let child=button.child().unwrap();
    let icon=if mode=="only"{child.clone()}else if mode=="leading"{child.first_child().unwrap()}else{child.last_child().unwrap()};
    assert!(icon.is::<gtk::DrawingArea>());assert_eq!(icon.accessible_role(),gtk::AccessibleRole::Presentation);
    assert_eq!(icon.width() as f64,expected["icon_width"].as_f64().unwrap()*scale);
    assert_eq!(icon.height() as f64,expected["icon_height"].as_f64().unwrap()*scale);
    if mode=="only" {
     let b=icon.compute_bounds(&button).unwrap();assert!((b.x()+b.width()/2.0-button_bounds.x()-button_bounds.width()/2.0).abs()<=0.5);
    } else {
     let first=child.first_child().unwrap().compute_bounds(&child).unwrap();let last=child.last_child().unwrap().compute_bounds(&child).unwrap();
     assert!((last.x()-first.x()-first.width()-expected["gap"].as_f64().unwrap() as f32*scale as f32).abs()<=0.5,"icon/text gap");
    }
    // Verify live theme foreground independently of the native token table.
    let color=icon.style_context().color();
    for (actual,wanted) in [color.red(),color.green(),color.blue()].iter().zip(expected["icon_rgb"].as_array().unwrap()) {assert!((*actual as f64*255.0-wanted.as_f64().unwrap()).abs()<=3.0,"{name} foreground {color:?}");}
    let bounds=icon.compute_bounds(&icon).unwrap();let snapshot=gtk::Snapshot::new();gtk::WidgetPaintable::new(Some(&icon)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
    let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().expect("icon must paint"),Some(&bounds));let mut pixels=vec![0;texture.width() as usize*texture.height() as usize*4];texture.download(&mut pixels,texture.width() as usize*4);let painted=pixels.chunks_exact(4).filter(|p|p[3]>=128).collect::<Vec<_>>();assert!(!painted.is_empty(),"icon is invisible");
    for pixel in painted {for (actual,wanted) in [pixel[2],pixel[1],pixel[0]].iter().zip(expected["icon_rgb"].as_array().unwrap()) {assert!((*actual as f64*255.0/pixel[3] as f64-wanted.as_f64().unwrap()).abs()<=3.0,"{name}/{icon_name} painted foreground");}}
    report.push(serde_json::json!({"size":size,"variant":name,"icon":icon_name,"width":button_bounds.width(),"height":button_bounds.height()}));
   }
   let bounds=panel.compute_bounds(&panel).unwrap();let snapshot=gtk::Snapshot::new();gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),Some(&bounds)).save_to_png(capture).unwrap();
   println!("{}",serde_json::json!({"result":"pass","mode":mode,"dark":dark,"scale":scale,"metrics":report}));quit.quit();
  });
 });
    app.run_with_args(&["716-icon-metrics"]);
}
