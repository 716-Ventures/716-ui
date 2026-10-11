//! Native action/clipboard regression; child reads through another GTK client.
use gtk::{gdk, gio, glib, prelude::*};
use seven_sixteen_ui::{gtk, DataTable, TableRow};
use std::{ffi::OsStr, time::Duration};

async fn read_current() -> String {
    gdk::Display::default()
        .unwrap()
        .clipboard()
        .read_text_future()
        .await
        .unwrap()
        .unwrap()
        .to_string()
}

async fn read_other_client() -> String {
    if std::env::var_os("TABLE_CLIPBOARD_LOCAL").is_some() {
        return gdk::Display::default()
            .unwrap()
            .clipboard()
            .read_text_future()
            .await
            .unwrap()
            .unwrap()
            .to_string();
    }
    let executable = std::env::current_exe().unwrap();
    let child = gio::Subprocess::newv(
        &[executable.as_os_str(), OsStr::new("--read")],
        gio::SubprocessFlags::STDOUT_PIPE,
    )
    .unwrap();
    let result =
        glib::future_with_timeout(Duration::from_secs(5), child.communicate_utf8_future(None))
            .await;
    if result.is_err() {
        child.force_exit();
    }
    let (output, _) = result.unwrap().unwrap();
    assert!(child.is_successful());
    serde_json::from_str(output.as_deref().unwrap().trim()).unwrap()
}

fn main() {
    let reader = std::env::args().any(|a| a == "--read");
    let app = gtk::Application::builder()
        .application_id(if reader {
            "com.sevensixteen.TableReader"
        } else {
            "com.sevensixteen.TableClipboard"
        })
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();
    app.connect_activate(move |app| {
        let panel = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let window = gtk::ApplicationWindow::builder().application(app).title("716 UI Table clipboard").child(&panel).default_width(360).build();
        window.present();
        let quit = app.clone();
        glib::MainContext::default().spawn_local(async move {
            glib::timeout_future(Duration::from_millis(300)).await;
            if reader {
                let text = window.clipboard().read_text_future().await.unwrap().unwrap();
                println!("{}",serde_json::to_string(&text.to_string()).unwrap());
                quit.quit();
                return;
            }
            let mut table = DataTable::new("Copy regression", &["Resource".into(), "Status".into()]).unwrap();
            let row = TableRow{id:"unicode".into(),cells:vec!["Document λ 日本語".into(),"Saved\nsecond line".into()]};
            table.update(&[row.clone()]).unwrap();
            panel.append(&table.widget);
            glib::timeout_future(Duration::from_millis(100)).await;
            table.view.clipboard().set_text("sentinel");
            table.view.activate_action("table.copy", None).unwrap();
            assert_eq!(read_current().await,"sentinel","No selection must preserve clipboard");
            assert!(table.select_key("unicode"));
            // Exercise the real installed capture callback, not host input.
            let controllers = table.view.observe_controllers();
            let keys = (0..controllers.n_items()).filter_map(|i|controllers.item(i).and_downcast::<gtk::EventControllerKey>()).find(|k|k.name().as_deref()==Some("seven-table-clipboard")).unwrap();
            let result = keys.emit_by_name::<bool>("key-pressed", &[&gdk::Key::c,&0u32,&gdk::ModifierType::CONTROL_MASK]);
            assert!(result);
            assert_eq!(read_current().await,"Document λ 日本語\tSaved\nsecond line");
            let updated = TableRow{id:"unicode".into(),cells:vec!["Updated λ 日本語".into(),"Current".into()]};
            table.update(&[TableRow{id:"first".into(),cells:vec!["Other".into(),"Row".into()]},updated]).unwrap();
            assert_eq!(table.selected_key().as_deref(),Some("unicode"));
            // Accessible menu invokes the same action after reconciliation.
            let menu = table.view.last_child().unwrap().downcast::<gtk::Popover>().unwrap();
            let item = menu.child().unwrap().downcast::<gtk::Button>().unwrap();
            assert_eq!(item.accessible_role(),gtk::AccessibleRole::MenuItem);
            assert_eq!(item.label().as_deref(),Some("Copy selected row"));
            assert!(keys.emit_by_name::<bool>("key-pressed", &[&gdk::Key::F10,&0u32,&gdk::ModifierType::SHIFT_MASK]));
            glib::timeout_future(Duration::from_millis(100)).await;
            assert!(menu.is_visible(),"Keyboard context menu did not open");
            item.emit_clicked();
            glib::timeout_future(Duration::from_millis(100)).await;
            assert!(!menu.is_visible(),"Copy left the context menu open");
            let focused = gtk::prelude::RootExt::focus(&window).unwrap();
            assert!(focused == *table.view.upcast_ref::<gtk::Widget>() || focused.is_ancestor(&table.view),"Copy did not restore local Table focus");
            assert_eq!(read_current().await,"Updated λ 日本語\tCurrent");
            table.update(&[]).unwrap();
            table.view.activate_action("table.copy",None).unwrap();
            assert_eq!(read_current().await,"Updated λ 日本語\tCurrent","Removed selection must preserve clipboard");
            assert_eq!(read_other_client().await,"Updated λ 日本語\tCurrent");
            panel.remove(&table.widget);
            let weak = table.view.downgrade();
            drop((keys,controllers,menu,item,table));
            glib::timeout_future(Duration::from_millis(100)).await;
            assert!(weak.upgrade().is_none(),"Copy/menu callbacks retain the table");
            println!("PASS: Table copy callback, explicit accessible menu, exact Unicode clipboard, reconciliation/removal and owner release; cross-client={}",std::env::var_os("TABLE_CLIPBOARD_LOCAL").is_none());
            quit.quit();
        });
    });
    app.run_with_args::<&str>(&[]);
}
