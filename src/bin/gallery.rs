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
        let mut list=seven_sixteen_ui::DataList::new("Observed items",false).unwrap();
        let mut list_rows=(0..4000).map(|i|seven_sixteen_ui::TableRow{id:format!("item-{i}"),cells:vec![format!("Item {i} · 日本語")]}).collect::<Vec<_>>();
        list.update(&list_rows).unwrap();content.append(&list.widget);
        let mut pairs=seven_sixteen_ui::DataList::new("Resource metadata",true).unwrap();
        pairs.update(&[seven_sixteen_ui::TableRow{id:"kind".into(),cells:vec!["Kind".into(),"File".into()]},seven_sixteen_ui::TableRow{id:"mode".into(),cells:vec!["Permissions".into(),"0644".into()]}]).unwrap();content.append(&pairs.widget);
        let section=seven_sixteen_ui::Section::new("Reading section").unwrap();
        let region=seven_sixteen_ui::ScrollRegion::new("Long reading region").unwrap();
        for i in 0..100 {region.content.append(&seven_sixteen_ui::text(&format!("Reading line {i} · 日本語"),false));}
        section.content.append(&region.widget);content.append(&section.widget);
        let tabs=seven_sixteen_ui::Tabs::new("Inspection pages").unwrap();
        let page_a=seven_sixteen_ui::text("First page 日本語",false).upcast::<gtk::Widget>();
        let page_b=seven_sixteen_ui::text("Second page λ",false).upcast::<gtk::Widget>();
        tabs.set_pages(&[("first".into(),"First".into(),page_a.clone()),("second".into(),"Second".into(),page_b.clone())]).unwrap();content.append(&tabs.widget);
        let split=seven_sixteen_ui::SplitView::new("Comparison panes",false).unwrap();
        let left=seven_sixteen_ui::text("Left comparison",false).upcast::<gtk::Widget>();let right=seven_sixteen_ui::text("Right comparison",false).upcast::<gtk::Widget>();
        split.set_children(&left,&right).unwrap();content.append(&split.widget);
        content.append(&seven_sixteen_ui::progress("Observed work",Some(0.65)));
        content.append(&seven_sixteen_ui::link("Component source and documentation","https://github.com/716-Ventures/716-ui").unwrap());
        let choice=seven_sixteen_ui::Choice::new("Reading density", &["Compact".into(),"Comfortable".into()], "Comfortable").unwrap();
        let choices=Rc::new(RefCell::new(Vec::new()));let observed=choices.clone();choice.on_changed(move |value|observed.borrow_mut().push(value));content.append(&choice.widget);
        let toggle=seven_sixteen_ui::Toggle::new("Wrap text",true).unwrap();let toggles=Rc::new(RefCell::new(Vec::new()));let observed=toggles.clone();toggle.on_changed(move |value|observed.borrow_mut().push(value));content.append(&toggle.widget);
        let rich=seven_sixteen_ui::RichText::new(&[seven_sixteen_ui::RichRun{text:"Literal <b>日本語</b> & λ".into(),style:"strong".into()}]).unwrap();content.append(&rich.widget);
        let mut chart=seven_sixteen_ui::BoundedChart::new("Measured comparison",&[seven_sixteen_ui::ChartPoint{id:"loss".into(),label:"Loss".into(),value:-12.5},seven_sixteen_ui::ChartPoint{id:"gain".into(),label:"Gain".into(),value:25.0}]).unwrap();content.append(&chart.widget);
        let image=seven_sixteen_ui::ImageView::new("Blue pixel fixture",include_bytes!("../../tests/pixel.png")).unwrap();content.append(&image.widget);
        let outcome=seven_sixteen_ui::OutcomeView::new("Completed work","Exact result 日本語",false).unwrap();content.append(&outcome.widget);
        let failure=seven_sixteen_ui::OutcomeView::new("Work unavailable","Literal <retry> & λ",true).unwrap();content.append(&failure.widget);
        failure.set_recovery("Retry assessed work",true).unwrap();let recovered=Rc::new(RefCell::new(0));let observed=recovered.clone();failure.on_recover(move||*observed.borrow_mut()+=1);
        #[cfg(feature="terminal")]
        let terminal=seven_sixteen_ui::TerminalView::new("Embedded terminal fixture").unwrap();
        #[cfg(feature="terminal")]
        {terminal.feed(b"\x1b[32mTerminal output\x1b[0m\r\nUnicode: \xce\xbb",false).unwrap();content.append(&terminal.widget);}
        let editor=seven_sixteen_ui::DocumentEditor::new("Shared document","日本語 λ 日本語").unwrap();content.append(&editor.widget);
        let reference=seven_sixteen_ui::ReferenceView::new("Open related view").unwrap();
        let opened=Rc::new(RefCell::new(0));let observed=opened.clone();reference.on_open(move ||*observed.borrow_mut()+=1);content.append(&reference.widget);
        let scroll=gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).child(&content).build();
        let window=gtk::ApplicationWindow::builder().application(app).title("716 UI component gallery").default_width(820).default_height(560).child(&scroll).build();window.add_css_class("seven-ui");let paintable=gtk::WidgetPaintable::new(Some(&content));window.present();primary.grab_focus();
        let quit=app.clone();let capture=capture.clone();
        if test || capture.is_some(){glib::timeout_add_local_once(Duration::from_millis(700),move||{
            if test {
                #[cfg(feature="terminal")]
                {assert!(!terminal.input_enabled());assert!(terminal.feed(&vec![0;16385],false).is_err());let (rows,cols)=terminal.grid();assert!(rows>0 && cols>0);terminal.set_attached(true);assert!(terminal.input_enabled());terminal.set_attached(false);}
                assert!(seven_sixteen_ui::DocumentEditor::new("Document",&"x".repeat(65537)).is_err());
                editor.find.set_text("日本語");editor.next.emit_clicked();assert!(editor.area.view.buffer().selection_bounds().is_some());
                editor.replacement.set_text("café");editor.replace.emit_clicked();assert_eq!(editor.area.value(),"café λ 日本語");
                editor.replace_all.emit_clicked();assert_eq!(editor.area.value(),"café λ café");editor.area.view.buffer().undo();assert_eq!(editor.area.value(),"café λ 日本語");
                editor.find.set_text(".*");editor.next.emit_clicked();assert_eq!(editor.notice.text(),"No literal match");
                let before=editor.area.value();editor.area.view.buffer().insert_at_cursor(&"x".repeat(65536));assert_eq!(editor.area.value(),before);assert!(editor.notice.text().contains("64 KiB"));
                assert_eq!(outcome.widget.accessible_role(),gtk::AccessibleRole::Status);
                assert_eq!(failure.widget.accessible_role(),gtk::AccessibleRole::Alert);
                assert_eq!(failure.message.text(),"Literal <retry> & λ");
                failure.message.select_region(0,7);assert!(!failure.reconcile("Changed","Other").unwrap());assert_eq!(failure.message.text(),"Literal <retry> & λ");
                failure.message.select_region(0,0);assert!(failure.reconcile("Recovery needed","Updated reason").unwrap());
                failure.recovery.emit_clicked();assert_eq!(*recovered.borrow(),1);
                failure.set_recovery("Retry assessed work",false).unwrap();assert!(!failure.recovery.is_sensitive());
                assert!(failure.set_recovery("",true).is_err());assert!(!failure.recovery.is_sensitive());
                assert!(seven_sixteen_ui::ReferenceView::new("").is_err());
                reference.widget.emit_clicked();assert_eq!(*opened.borrow(),1);
                reference.reconcile("View unavailable",false).unwrap();assert!(!reference.widget.is_sensitive());
                assert!(reference.reconcile("",true).is_err());assert!(!reference.widget.is_sensitive());
                reference.reconcile("Open related view",true).unwrap();assert!(reference.widget.is_sensitive());
                assert_eq!(reading.layout().unknown_glyphs_count(),0,"Missing Unicode fallback font");
                assert_eq!(heading.accessible_role(),gtk::AccessibleRole::Heading);
                assert_eq!(section.widget.accessible_role(),gtk::AccessibleRole::Group);
                assert_eq!(section.heading.accessible_role(),gtk::AccessibleRole::Heading);
                assert!(region.widget.vadjustment().upper()>region.widget.vadjustment().page_size());
                region.widget.vadjustment().set_value(100.0);assert!(region.widget.vadjustment().value()>0.0);
                section.set_label("Updated section").unwrap();assert_eq!(section.heading.text(),"Updated section");
                assert!(section.set_label(" ").is_err());
                assert!(tabs.select_key("second"));
                tabs.set_pages(&[("second".into(),"Renamed second".into(),page_b.clone()),("first".into(),"First".into(),page_a.clone())]).unwrap();
                assert_eq!(tabs.selected_key().as_deref(),Some("second"));assert_eq!(tabs.widget.tab_label_text(&page_b).as_deref(),Some("Renamed second"));
                assert!(tabs.set_pages(&[("same".into(),"A".into(),page_a.clone()),("same".into(),"B".into(),page_b.clone())]).is_err());
                assert_eq!(tabs.selected_key().as_deref(),Some("second"));
                tabs.set_pages(&[("first".into(),"First".into(),page_a.clone())]).unwrap();assert_eq!(tabs.selected_key().as_deref(),Some("first"));
                assert!(tabs.set_pages(&[("foreign".into(),"Foreign".into(),left.clone())]).is_err());
                split.widget.set_position(120);let position=split.widget.position();split.set_children(&left,&right).unwrap();assert_eq!(split.widget.position(),position);
                split.clear();split.set_children(&right,&left).unwrap();assert_eq!(split.children(),vec![right.clone(),left.clone()]);
                assert!(split.set_children(&left,&left).is_err());assert_eq!(split.children(),vec![right.clone(),left.clone()]);
                choice.control.set_selected(0);assert_eq!(choice.value().as_deref(),Some("Compact"));assert_eq!(*choices.borrow(),vec!["Compact".to_string()]);
                choice.control.grab_focus();assert!(!choice.reconcile("Comfortable"));assert_eq!(choice.value().as_deref(),Some("Compact"));
                assert!(seven_sixteen_ui::Choice::new("Invalid", &["A".into(),"A".into()], "A").is_err());
                assert!(seven_sixteen_ui::Choice::new("Invalid", &["A".into()], "B").is_err());
                toggle.control.set_active(false);assert_eq!(*toggles.borrow(),vec![false]);toggle.control.grab_focus();assert!(!toggle.reconcile(true));assert!(!toggle.control.is_active());
                assert_eq!(toggle.control.accessible_role(),gtk::AccessibleRole::Checkbox);
                assert_eq!(rich.widget.text(),"Literal <b>日本語</b> & λ");rich.widget.select_region(0,-1);
                assert!(!rich.reconcile(&[seven_sixteen_ui::RichRun{text:"Replacement".into(),style:"plain".into()}]).unwrap());assert_eq!(rich.widget.text(),"Literal <b>日本語</b> & λ");
                assert_eq!(chart.data.row_count(),2);assert!(chart.plot.height()>0);assert!(chart.plot.width()>0);
                assert!(chart.update("Invalid",&[seven_sixteen_ui::ChartPoint{id:"a".into(),label:"A".into(),value:f64::NAN}]).is_err());assert_eq!(chart.data.row_count(),2);
                chart.update("Updated measured comparison",&[seven_sixteen_ui::ChartPoint{id:"gain".into(),label:"Gain".into(),value:0.0}]).unwrap();assert_eq!(chart.data.row_count(),1);
                assert!(image.picture.paintable().is_some());
                image.set_label("Updated image alternative").unwrap();assert_eq!(image.caption.text(),"Updated image alternative");assert!(image.set_label(" ").is_err());
                assert!(seven_sixteen_ui::ImageView::new("Invalid",b"not an image").is_err());
                assert_eq!(primary.accessible_role(),gtk::AccessibleRole::Button);assert!(!disabled.is_sensitive());
                primary.emit_clicked();assert_eq!(*clicks.borrow(),1);
                reading.select_region(0,-1);assert!(reading.selection_bounds().is_some());
                field.entry.set_text("Unicode draft λ 日本語");field.entry.emit_activate();
                assert!(events.borrow().contains(&InputEvent::Submitted("Unicode draft λ 日本語".into())));
                field.entry.grab_focus();assert!(!field.reconcile("Stale external text"));
                assert_eq!(field.entry.text(),"Unicode draft λ 日本語");assert!(window.width()>0 && primary.height()>0);
                assert_eq!(field.entry.accessible_role(),gtk::AccessibleRole::TextBox);
                area.view.grab_focus();area.view.buffer().set_text("Multiline λ\nSecond line");assert!(!area.reconcile("External replacement"));assert_eq!(area.value(),"Multiline λ\nSecond line");assert_eq!(area.view.accessible_role(),gtk::AccessibleRole::TextBox);
                scroll.vadjustment().set_value(scroll.vadjustment().upper());
                for _ in 0..32 {if glib::MainContext::default().pending(){glib::MainContext::default().iteration(false);}else{break;}}
                assert!(table.live_cell_count()>0);assert!(table.view.height()>0);
                assert_eq!(table.row_count(),4000);assert!(table.live_cell_count()<1000,"Collection widgets must be virtualized");
                assert!(table.select_key("row-7"));table_rows.reverse();table_rows[3992].cells[1]="Updated".into();table.update(&table_rows).unwrap();
                assert_eq!(table.selected_key().as_deref(),Some("row-7"));
                let previous=table.row_count();let mut duplicate=table_rows.clone();duplicate[0].id=duplicate[1].id.clone();assert!(table.update(&duplicate).is_err());assert_eq!(table.row_count(),previous);
                assert!(list.live_row_count()>0 && list.live_row_count()<500);assert_eq!(list.row_count(),4000);
                assert_eq!(list.view.accessible_role(),gtk::AccessibleRole::List);
                assert!(list.select_key("item-7"));list_rows.reverse();list.update(&list_rows).unwrap();assert_eq!(list.selected_key().as_deref(),Some("item-7"));
                list_rows.retain(|r|r.id!="item-7");list.update(&list_rows).unwrap();assert!(list.selected_key().is_none());
                assert!(list.update(&[seven_sixteen_ui::TableRow{id:"wrong".into(),cells:vec!["one".into(),"two".into()]}]).is_err());
                assert_eq!(pairs.row_count(),2);assert!(pairs.live_row_count()>0);
                assert!(seven_sixteen_ui::DataList::new(" ",false).is_err());
                assert!(seven_sixteen_ui::link("Unsafe","javascript:run()").is_err());
                println!("PASS: native rendering, selectable Unicode, typed form events, focus-preserved draft, disabled state, accessibility roles, safe link schemes");
            }
            glib::timeout_add_local_once(Duration::from_millis(180),move||{
            if test {assert!(image.picture.height()>0 && image.picture.height()<=240,"Visible image height {}",image.picture.height());}
            if let Some(path)=capture {
                let snapshot=gtk::Snapshot::new();
                paintable.snapshot(&snapshot,content.width() as f64,content.height() as f64);
                let node=snapshot.to_node().expect("A rendered scene");
                let texture=window.renderer().unwrap().render_texture(&node,None);texture.save_to_png(&path).expect("Save native capture");
                println!("CAPTURE: {path}");
            }
            quit.quit();
            });
        });}
    });
    app.run_with_args(&["716-ui-gallery"]);
}
