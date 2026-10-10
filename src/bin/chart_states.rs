//! Real fractional/signed chart rendering, independent of domain calculations.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, BoundedChart, ChartPoint};
use std::time::Duration;
fn main() {
    let dark=std::env::args().any(|arg|arg=="--dark");
    let case=std::env::var("CHART_CASE").unwrap();
    let output=std::env::var("CHART_STATE_OUTPUT").unwrap();
    let values=match case.as_str() {
        "positive"=>[0.1,0.2], "negative"=>[-0.1,-0.2], "mixed"=>[-0.1,0.2],
        "zero"=>[0.,0.], "tiny"=>[1e-12,2e-12], "subnormal"=>[f64::from_bits(1),f64::from_bits(2)], "integer"=>[1.,2.], _=>panic!("Unknown chart fixture"),
    };
    let app=gtk::Application::builder().application_id("com.sevensixteen.ChartStates").build();
    app.connect_activate(move |app| {
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark {Appearance::Dark}else{Appearance::Light},1.,true);
        let points=values.into_iter().enumerate().map(|(i,value)|ChartPoint{id:format!("point-{i}"),label:format!("Measured {i} λ 日本語"),value}).collect::<Vec<_>>();
        let chart=BoundedChart::new("Measured values",&points).unwrap();
        let panel=gtk::Box::new(gtk::Orientation::Vertical,0);panel.add_css_class("seven-ui");panel.append(&chart.widget);
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI chart scale").child(&panel).build();
        window.present();let quit=app.clone();let output=output.clone();let case=case.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move || {
            let plot=chart.plot;let bounds=plot.compute_bounds(&plot).unwrap();
            let bg=if dark {[9,9,11]}else{[255,255,255]};let fg=if dark {[250,250,250]}else{[9,9,11]};
            let snapshot=gtk::Snapshot::new();snapshot.append_color(&gdk::RGBA::new(bg[0] as f32/255.,bg[1] as f32/255.,bg[2] as f32/255.,1.),&bounds);
            gtk::WidgetPaintable::new(Some(&plot)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
            let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),None);
            std::fs::create_dir_all(&output).unwrap();texture.save_to_png(format!("{output}/{case}.png")).unwrap();
            let stride=texture.width() as usize*4;let mut pixels=vec![0;stride*texture.height() as usize];texture.download(&mut pixels,stride);
            let (x,y)=if case=="mixed" {(0.25,0.9)}else{(0.75,0.75)};
            let i=(texture.height() as f64*y) as usize*stride+(texture.width() as f64*x) as usize*4;
            let actual=[pixels[i+2],pixels[i+1],pixels[i]];
            let expected=if case=="zero" {bg}else{std::array::from_fn(|i|(fg[i] as f64*0.85+bg[i] as f64*0.15).round() as u8)};
            println!("{}",serde_json::json!({"case":case,"dark":dark,"painted_rgb":actual,"expected_rgb":expected,"width":texture.width(),"height":texture.height()}));
            assert!(actual.iter().zip(expected).all(|(a,b)|a.abs_diff(b)<=2),"{case}: bars did not use the available zero-based domain");
            assert_eq!(chart.data.row_count(),2);quit.quit();
        });
    });app.run_with_args::<&str>(&[]);
}
