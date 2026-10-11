//! Selectable status must expose fresh observations without erasing selection.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, StatusView, TextField};
use std::time::Duration;

fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let rtl = std::env::args().any(|arg| arg == "--rtl");
    let scale = if std::env::args().any(|arg| arg == "--large") { 2. } else { 1. };
    let app = gtk::Application::builder().application_id("com.sevensixteen.StatusUpdates").build();
    let completed = std::rc::Rc::new(std::cell::Cell::new(false));
    let verified = completed.clone();
    app.connect_activate(move |app| {
        let completed = verified.clone();
        let watchdog = app.clone();
        glib::timeout_add_local_once(Duration::from_secs(8), move || watchdog.quit());
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(), if dark {Appearance::Dark} else {Appearance::Light}, scale, true);
        assert!(StatusView::new("literal\0value").is_err());
        assert!(StatusView::new(&"λ".repeat(32769)).is_err());
        assert!(StatusView::new(&"λ".repeat(32768)).is_ok());
        let status = StatusView::new("running λ").unwrap();
        let draft = TextField::new("Unsent draft", "", "Keep this draft 日本語");
        let panel = seven_sixteen_ui::column(16);
        panel.add_css_class("seven-ui");
        panel.set_direction(if rtl {gtk::TextDirection::Rtl} else {gtk::TextDirection::Ltr});
        panel.set_margin_start(24);panel.set_margin_end(24);
        panel.set_margin_top(24);panel.set_margin_bottom(24);
        panel.append(&seven_sixteen_ui::text("Observed work", true));
        panel.append(&status.widget);panel.append(&draft.widget);
        let window = gtk::ApplicationWindow::builder().application(app).child(&panel).default_width(520).default_height(380).build();
        window.present();
        let quit = app.clone();
        glib::MainContext::default().spawn_local(async move {
            glib::timeout_future(Duration::from_millis(300)).await;
            assert_eq!(status.widget.accessible_role(), gtk::AccessibleRole::Status);
            assert!(status.value.is_selectable());
            assert!(status.value.grab_focus());
            status.value.select_region(0, -1);
            let selection = status.value.selection_bounds();
            assert!(selection.is_some());
            assert!(!status.reconcile("running λ").unwrap());
            assert_eq!(status.value.selection_bounds(), selection);
            assert!(!status.current.is_visible());
            assert!(!status.reconcile("failed 日本語").unwrap());
            glib::timeout_future(Duration::from_millis(100)).await;
            assert_eq!(status.value.text(), "running λ");
            assert_eq!(status.value.selection_bounds(), selection);
            assert_eq!(gtk::prelude::RootExt::focus(&window), Some(status.value.clone().upcast::<gtk::Widget>()));
            assert!(status.current.is_mapped());
            assert!(!status.current.is_selectable());
            assert_eq!(status.current.text(), "Current value:\nfailed 日本語");
            assert_eq!(draft.entry.text(), "Keep this draft 日本語");
            for invalid in ["bad\0value".to_owned(), "x".repeat(65537)] {
                assert!(status.reconcile(&invalid).is_err());
                assert_eq!(status.value.text(), "running λ");
                assert_eq!(status.current.text(), "Current value:\nfailed 日本語");
                assert_eq!(status.value.selection_bounds(), selection);
            }
            if let Ok(path) = std::env::var("STATUS_UPDATE_OUTPUT") {
                let paintable = gtk::WidgetPaintable::new(Some(&window));
                let snapshot = gtk::Snapshot::new();
                paintable.snapshot(&snapshot, window.width() as f64, window.height() as f64);
                window.renderer().unwrap().render_texture(&snapshot.to_node().unwrap(), None).save_to_png(path).unwrap();
            }
            assert!(!status.reconcile("stopped Ελληνικά").unwrap());
            assert_eq!(status.current.text(), "Current value:\nstopped Ελληνικά");
            assert_eq!(status.value.selection_bounds(), selection);
            status.value.select_region(0, 0);
            assert!(status.reconcile("stopped Ελληνικά").unwrap());
            glib::timeout_future(Duration::from_millis(100)).await;
            assert_eq!(status.value.text(), "stopped Ελληνικά");
            assert!(!status.current.is_mapped());
            assert_eq!(gtk::prelude::RootExt::focus(&window), Some(status.value.clone().upcast::<gtk::Widget>()));
            assert_eq!(draft.entry.text(), "Keep this draft 日本語");
            assert!(status.reconcile("").unwrap());
            assert_eq!(status.value.text(), "");
            println!("PASS: mapped selectable Status preserves Unicode selection, logical focus and unsent draft; current observations stay visible; invalid updates are atomic; deselection and empty state reconcile");
            completed.set(true);window.destroy();quit.quit();
        });
    });
    app.run_with_args::<&str>(&[]);
    assert!(completed.get(), "Status native verification did not complete");
}
