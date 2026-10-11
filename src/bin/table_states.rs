//! Native read-only table endpoints and bounded horizontal overflow.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, DataTable, TableRow};
use std::time::Duration;
fn descendants(widget: &gtk::Widget, name: &str) -> Vec<gtk::Widget> {
    let mut found = Vec::new();
    let mut pending = vec![widget.clone()];
    while let Some(widget) = pending.pop() {
        if widget.css_name() == name && widget.is_mapped() && widget.height() > 0 {
            found.push(widget.clone());
        }
        let mut child = widget.first_child();
        while let Some(current) = child {
            pending.push(current.clone());
            child = current.next_sibling();
        }
    }
    found
}
fn main() {
    let dark = std::env::args().any(|a| a == "--dark");
    let output = std::env::var("TABLE_STATE_OUTPUT").unwrap();
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.TableStates")
        .build();
    app.connect_activate(move |app| {
        let reference:serde_json::Value=serde_json::from_str(include_str!("../../reference/table-observation.json")).unwrap();
        let profile:serde_json::Value=serde_json::from_str(include_str!("../../reference/profile.json")).unwrap();
        assert_eq!(reference["component_sha256"],profile["files"]["table.tsx"]);
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(),if dark {Appearance::Dark}else{Appearance::Light},1.,true);
        let panel=gtk::Box::new(gtk::Orientation::Vertical,12);panel.add_css_class("seven-ui");
        panel.set_margin_top(20);panel.set_margin_bottom(20);panel.set_margin_start(20);panel.set_margin_end(20);
        let sink=gtk::Button::with_label("Focus sink");panel.append(&sink);
        let mut table=DataTable::new("Reference table",&["Resource".into(),"Status".into()]).unwrap();
        table.update(&[("row-0","Document λ","Saved"),("row-1","Background job","Running"),("row-2","External app","Observed")].into_iter().map(|(id,first,second)|TableRow{id:id.into(),cells:vec![first.into(),second.into()]}).collect::<Vec<_>>()).unwrap();
        table.widget.set_size_request(360,-1);panel.append(&table.widget);
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI Table states").default_width(400).child(&panel).build();window.present();sink.grab_focus();
        let output=output.clone();let quit=app.clone();
        glib::timeout_add_local_once(Duration::from_millis(500),move || {
            assert!(table.select_key("row-2"));
            let mut rows=descendants(table.view.upcast_ref(),"row");
            rows.sort_by_key(|row|row.compute_bounds(&panel).unwrap().y() as i32);assert_eq!(rows.len(),3);
            rows[1].set_state_flags(gtk::StateFlags::PRELIGHT,false);
            glib::timeout_add_local_once(Duration::from_millis(100),move || {
                let mut failures=Vec::new();let bounds=panel.compute_bounds(&panel).unwrap();let snapshot=gtk::Snapshot::new();
                gtk::WidgetPaintable::new(Some(&panel)).snapshot(&snapshot,bounds.width() as f64,bounds.height() as f64);
                let texture=window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(),None);
                std::fs::create_dir_all(&output).unwrap();texture.save_to_png(format!("{output}/table.png")).unwrap();
                let stride=texture.width() as usize*4;let mut pixels=vec![0;stride*texture.height() as usize];texture.download(&mut pixels,stride);
                let painted=|r:&gtk::graphene::Rect| {let i=(r.y()+r.height()/2.).round() as usize*stride+(r.x()+r.width()-20.).round() as usize*4;[pixels[i+2],pixels[i+1],pixels[i]]};
                let headers=descendants(table.view.upcast_ref(),"header");assert_eq!(headers.len(),1);
                let header=headers[0].compute_bounds(&panel).unwrap();let header_rgb=painted(&header);
                let page=if dark {[9,9,11]}else{[255,255,255]};
                println!("{}",serde_json::json!({"dark":dark,"header_height":header.height(),"header_rgb":header_rgb,"expected_rgb":page}));
                if (header.height()-40.).abs()>1. {failures.push("header geometry".into());}
                if !header_rgb.into_iter().zip(page).all(|(a,b)|a.abs_diff(b)<=2) {failures.push("header background".into());}
                for (index,row) in rows.iter().enumerate() {
                    let r=row.compute_bounds(&panel).unwrap();let rgb=painted(&r);
                    let expected=match (dark,index) {(false,0)=>[255,255,255],(false,1)=>[250,250,250],(false,_)=>[244,244,245],(true,0)=>[9,9,11],(true,1)=>[24,24,27],(true,_)=>[39,39,42]};
                    println!("{}",serde_json::json!({"dark":dark,"row":index,"height":r.height(),"painted_rgb":rgb,"expected_rgb":expected}));
                    if (r.height()-37.).abs()>1. {failures.push(format!("row {index} geometry"));}
                    if !rgb.into_iter().zip(expected).all(|(a,b)|a.abs_diff(b)<=2) {failures.push(format!("row {index} background"));}
                }
                assert_eq!(table.selected_key().as_deref(),Some("row-2"));
                table.update(&[TableRow{id:"wide".into(),cells:vec!["x ".repeat(2048),"Unicode λ 日本語".into()]}]).unwrap();
                glib::timeout_add_local_once(Duration::from_millis(200),move || {
                    let scroll=table.widget.hadjustment();let rows=descendants(table.view.upcast_ref(),"row");
                    let heights=rows.iter().map(|r|r.height()).collect::<Vec<_>>();
                    println!("{}",serde_json::json!({"dark":dark,"wide_cell_bytes":4096,"case":"spaced text remains one horizontal row","horizontal_upper":scroll.upper(),"horizontal_page_size":scroll.page_size(),"row_heights":heights}));
                    assert_eq!(heights.len(),1,"Wide row never mapped");
                    if scroll.upper()<=scroll.page_size()+1. {failures.push("wide cells have no horizontal overflow".into());}
                    if heights.iter().any(|h|*h>38) {failures.push("wide cells wrap into tall rows".into());}
                    assert!(failures.is_empty(),"Table reference endpoints/overflow: {failures:?}");quit.quit();
                });
            });
        });
    });
    app.run_with_args::<&str>(&[]);
}
