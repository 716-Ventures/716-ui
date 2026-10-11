//! Verify painted/accessible labels, not only the updated model contents.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, DataList, DataTable, TableRow};
use std::time::Duration;

fn labels(root: &gtk::Widget) -> Vec<String> {
    let mut pending = vec![root.clone()];
    let mut labels = Vec::new();
    while let Some(widget) = pending.pop() {
        if widget.is_mapped() {
            if let Some(label) = widget.downcast_ref::<gtk::Label>() {
                labels.push(label.text().to_string());
            }
        }
        let mut child = widget.first_child();
        while let Some(widget) = child {
            child = widget.next_sibling();
            pending.push(widget);
        }
    }
    labels
}

fn main() {
    let dark = std::env::args().any(|value| value == "--dark");
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.CollectionUpdates")
        .build();
    app.connect_activate(move |app| {
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(), if dark {Appearance::Dark} else {Appearance::Light}, 1., true);
        let panel = gtk::Box::new(gtk::Orientation::Vertical, 0);
        panel.add_css_class("seven-ui");
        let mut table = DataTable::new("Changing Table", &["Resource".into(), "Status".into()]).unwrap();
        let mut list = DataList::new("Changing List", false).unwrap();
        let mut values = DataList::new("Changing values", true).unwrap();
        let rows = vec![TableRow{id:"document".into(),cells:vec!["Original λ".into(),"Saved".into()]},TableRow{id:"other".into(),cells:vec!["Other row".into(),"Observed".into()]}];
        table.update(&rows).unwrap(); values.update(&rows).unwrap();
        list.update(&rows.iter().map(|row|TableRow{id:row.id.clone(),cells:vec![row.cells[0].clone()]}).collect::<Vec<_>>()).unwrap();
        for widget in [&table.widget,&list.widget,&values.widget] {panel.append(widget);}
        let window = gtk::ApplicationWindow::builder().application(app).child(&panel).default_width(500).default_height(600).build();
        window.present();
        let quit = app.clone();
        glib::MainContext::default().spawn_local(async move {
            glib::timeout_future(Duration::from_millis(300)).await;
            for widget in [&table.widget,&list.widget,&values.widget] {
                assert!(labels(widget.upcast_ref()).contains(&"Original λ".into()),"Initial collection label was not mapped");
            }
            assert!(table.select_key("document"));assert!(list.select_key("document"));assert!(values.select_key("document"));
            let changed = vec![rows[1].clone(),TableRow{id:"document".into(),cells:vec!["Updated λ 日本語".into(),"Current".into()]}];
            table.update(&changed).unwrap();values.update(&changed).unwrap();
            list.update(&changed.iter().map(|row|TableRow{id:row.id.clone(),cells:vec![row.cells[0].clone()]}).collect::<Vec<_>>()).unwrap();
            glib::timeout_future(Duration::from_millis(200)).await;
            let mut failures = Vec::new();
            for (name,widget,key) in [("Table",&table.widget,table.selected_key()),("List",&list.widget,list.selected_key()),("KeyValue",&values.widget,values.selected_key())] {
                let visible = labels(widget.upcast_ref());
                println!("{}",serde_json::json!({"collection":name,"labels":visible,"selected_key":key}));
                if !visible.contains(&"Updated λ 日本語".into()) || visible.contains(&"Original λ".into()) {failures.push(format!("{name} displays stale row text"));}
                assert_eq!(key.as_deref(),Some("document"));
            }
            assert!(failures.is_empty(),"Native collection update: {failures:?}");
            println!("PASS: mapped Table/List/KeyValue labels refresh for changed stable keys, retain unchanged rows and selected identity after reorder");
            quit.quit();
        });
    });
    app.run_with_args::<&str>(&[]);
}
