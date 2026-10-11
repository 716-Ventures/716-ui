//! Mapped text metrics survive GTK context replacement and live theme changes.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, ButtonSize, ButtonVariant};
use std::{cell::Cell, rc::Rc, time::Duration};

fn main() {
    let app=gtk::Application::builder().application_id("com.sevensixteen.ButtonFontReflow").build();
    app.connect_activate(|app| {
        let display=gdk::Display::default().unwrap();
        seven_sixteen_ui::install_theme(&display,Appearance::Light,1.0,true);
        let row=gtk::Box::new(gtk::Orientation::Horizontal,12);row.add_css_class("seven-ui");
        let mut samples=Vec::new();
        let clicks=Rc::new(Cell::new(0));
        for (variant,name) in [(ButtonVariant::Primary,"default"),(ButtonVariant::Outline,"outline")] {
            let button=seven_sixteen_ui::button_sized(name,variant,ButtonSize::ExtraSmall,false);
            button.set_valign(gtk::Align::Center);
            let count=clicks.clone();button.connect_clicked(move |_|count.set(count.get()+1));
            row.append(&button);samples.push((name,button));
        }
        let mut icons=Vec::new();
        for position in [None,Some(seven_sixteen_ui::IconPosition::Leading),Some(seven_sixteen_ui::IconPosition::Trailing)] {
            let button=if let Some(position)=position {seven_sixteen_ui::button_with_icon("default",seven_sixteen_ui::Icon::Plus,position,ButtonVariant::Primary,ButtonSize::ExtraSmall,false)}else{seven_sixteen_ui::icon_button("Add item",seven_sixteen_ui::Icon::Plus,ButtonVariant::Primary,ButtonSize::ExtraSmall,false)}.unwrap();
            button.set_valign(gtk::Align::Center);row.append(&button);icons.push((position,button));
        }
        let window=gtk::ApplicationWindow::builder().application(app).child(&row).build();window.present();
        let phase=Cell::new(0);let quit=app.clone();
        glib::timeout_add_local(Duration::from_millis(500),move || {
            let current=phase.get();let scale=if current==1 {2.0} else {1.0};
            let reference:serde_json::Value=serde_json::from_str(include_str!("../../reference/button-size-metrics.json")).unwrap();
            for (name,button) in &samples {
                let expected=reference["metrics"].as_array().unwrap().iter().find(|entry|entry["size"]=="xs"&&entry["variant"]==*name).unwrap();
                let label=button.child().unwrap().downcast::<gtk::Label>().unwrap();
                assert!(!label.pango_context().is_round_glyph_positions(),"Integral glyph positions returned after mapping/theme change");
                let border=if *name=="outline" {2.0} else {0.0};
                let width=(expected["width"].as_f64().unwrap()-border)*scale+border;
                let bounds=button.compute_bounds(button).unwrap();
                assert!((bounds.width() as f64-width).abs()<=scale,"{name} phase {current}: {}, expected {width}",bounds.width());
                assert_eq!(bounds.height() as f64,24.0*scale);
            }
            for (position,button) in &icons {
                let content=button.child().unwrap();
                let icon=if position.is_none(){content.clone()}else if *position==Some(seven_sixteen_ui::IconPosition::Leading){content.first_child().unwrap()}else{content.last_child().unwrap()};
                let bounds=button.compute_bounds(button).unwrap();
                assert_eq!(icon.width() as f64,12.0*scale);assert_eq!(icon.height() as f64,12.0*scale);
                assert_eq!(bounds.height() as f64,24.0*scale);
                if position.is_none(){assert_eq!(bounds.width() as f64,24.0*scale);}else{assert!((bounds.width() as f64-68.140625*scale).abs()<=scale);}
                let color=icon.style_context().color();let expected=if current==1{24.0}else{250.0};assert!((color.red() as f64*255.0-expected).abs()<=1.0);
            }
            assert_eq!(clicks.get(),0,"Theme/font reconciliation activated a button");
            match current {
                0=>seven_sixteen_ui::install_theme(&display,Appearance::Dark,2.0,true),
                1=>seven_sixteen_ui::install_theme(&display,Appearance::Light,1.0,true),
                _=>{println!("PASS: mapped fractional glyphs survive light-1 / dark-2 / light-1 theme changes without activation");quit.quit();return glib::ControlFlow::Break;}
            }
            phase.set(current+1);glib::ControlFlow::Continue
        });
    });
    app.run_with_args(&["716-ui-button-font-reflow"]);
}
