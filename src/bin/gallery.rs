use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonVariant, InputEvent, TextField};
use std::{cell::RefCell, rc::Rc, time::Duration};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let test = args.iter().any(|s| s == "--self-test");
    let capture = args
        .iter()
        .position(|s| s == "--capture")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let dark = args.iter().any(|s| s == "--dark");
    let reduced = args.iter().any(|s| s == "--reduced-motion");
    let scale = args
        .iter()
        .position(|s| s == "--scale")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.Components")
        .build();
    app.connect_activate(move|app|{
        let display=gdk::Display::default().expect("A native display is required");
        seven_sixteen_ui::install_theme(&display,if dark{Appearance::Dark}else{Appearance::Light},scale,reduced);
        let content=seven_sixteen_ui::column(20);content.add_css_class("seven-ui");
        for side in [gtk::PositionType::Top,gtk::PositionType::Bottom,gtk::PositionType::Left,gtk::PositionType::Right] {match side{
            gtk::PositionType::Top=>content.set_margin_top(28),gtk::PositionType::Bottom=>content.set_margin_bottom(28),gtk::PositionType::Left=>content.set_margin_start(28),_=>content.set_margin_end(28)
        }}
        let heading=seven_sixteen_ui::text("716 UI · Native components",true);content.append(&heading);
        let reading=seven_sixteen_ui::text("Selectable text: café · 日本語 · an independent native library. Keyboard, clipboard and editing run locally.",false);content.append(&reading);
        let buttons=seven_sixteen_ui::row(10);
        let primary=seven_sixteen_ui::button("Continue",ButtonVariant::Primary,false);buttons.append(&primary);
        buttons.append(&seven_sixteen_ui::button("Secondary",ButtonVariant::Secondary,false));
        buttons.append(&seven_sixteen_ui::button("Outline",ButtonVariant::Outline,false));
        let disabled=seven_sixteen_ui::button("Unavailable",ButtonVariant::Primary,true);buttons.append(&disabled);content.append(&buttons);
        let field=TextField::new("Your draft","Type here; Enter emits a typed event","An editable draft");content.append(&field.widget);
        let area=seven_sixteen_ui::TextArea::new("Multiline draft","First line\nSecond line · 日本語");content.append(&area.widget);
        let status=seven_sixteen_ui::status("Ready · no account or network needed");content.append(&status);
        let events=Rc::new(RefCell::new(Vec::new()));let recorded=events.clone();let live=status.clone();
        field.on_event(move|event|{live.set_text(match &event{InputEvent::Changed(_)=>"Editing locally",InputEvent::Submitted(_)=>"Submitted to the gallery"});recorded.borrow_mut().push(event);});
        let clicks=Rc::new(RefCell::new(0));let counted=clicks.clone();primary.connect_clicked(move |_|*counted.borrow_mut()+=1);
        let mut table=seven_sixteen_ui::DataTable::new("Observed resources",&["Resource".into(),"Status".into()]).unwrap();
        let mut table_rows=(0..4000).map(|i|seven_sixteen_ui::TableRow{id:format!("row-{i}"),cells:vec![format!("Resource {i} · 日本語"),"Observed".into()]}).collect::<Vec<_>>();
        table.update(&table_rows).unwrap();content.append(&table.widget);
        content.append(&seven_sixteen_ui::progress("Observed work",Some(0.65)));
        content.append(&seven_sixteen_ui::link("Component source and documentation","https://github.com/716-Ventures/716-ui").unwrap());
        let scroll=gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).child(&content).build();
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI component gallery").default_width(820).default_height(560).child(&scroll).build();window.add_css_class("seven-ui");let paintable=gtk::WidgetPaintable::new(Some(&content));window.present();primary.grab_focus();
        let quit=app.clone();let capture=capture.clone();
        if test || capture.is_some(){glib::timeout_add_local_once(Duration::from_millis(700),move||{
            if test {
                assert_eq!(reading.layout().unknown_glyphs_count(),0,"Missing Unicode fallback font");
                assert_eq!(primary.accessible_role(),gtk::AccessibleRole::Button);assert!(!disabled.is_sensitive());
                primary.emit_clicked();assert_eq!(*clicks.borrow(),1);
                reading.select_region(0,-1);assert!(reading.selection_bounds().is_some());
                field.entry.set_text("Unicode draft λ 日本語");field.entry.emit_activate();
                assert!(events.borrow().contains(&InputEvent::Submitted("Unicode draft λ 日本語".into())));
                field.entry.grab_focus();assert!(!field.reconcile("Stale external text"));
                assert_eq!(field.entry.text(),"Unicode draft λ 日本語");assert!(window.width()>0 && primary.height()>0);
                assert_eq!(field.entry.accessible_role(),gtk::AccessibleRole::TextBox);
                area.view.grab_focus();area.view.buffer().set_text("Multiline λ\nSecond line");assert!(!area.reconcile("External replacement"));assert_eq!(area.value(),"Multiline λ\nSecond line");assert_eq!(area.view.accessible_role(),gtk::AccessibleRole::TextBox);
                assert_eq!(table.row_count(),4000);assert!(table.live_cell_count()<1000,"Collection widgets must be virtualized");
                assert!(table.select_key("row-7"));table_rows.reverse();table_rows[3992].cells[1]="Updated".into();table.update(&table_rows).unwrap();
                assert_eq!(table.selected_key().as_deref(),Some("row-7"));
                let previous=table.row_count();let mut duplicate=table_rows.clone();duplicate[0].id=duplicate[1].id.clone();assert!(table.update(&duplicate).is_err());assert_eq!(table.row_count(),previous);
                assert!(seven_sixteen_ui::link("Unsafe","javascript:run()").is_err());
                println!("PASS: native rendering, selectable Unicode, typed form events, focus-preserved draft, disabled state, accessibility roles, safe link schemes");
            }
            if let Some(path)=capture {
                let snapshot=gtk::Snapshot::new();
                paintable.snapshot(&snapshot,content.width() as f64,content.height() as f64);
                let node=snapshot.to_node().expect("A rendered scene");
                let texture=window.renderer().unwrap().render_texture(&node,None);texture.save_to_png(&path).expect("Save native capture");
                println!("CAPTURE: {path}");
            }
            quit.quit();
        });}
    });
    app.run_with_args(&["716-ui-gallery"]);
}
