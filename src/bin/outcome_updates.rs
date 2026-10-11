//! Selected text must not disguise the current state of a live outcome.
use gtk::{gdk, glib, prelude::*};
use seven_sixteen_ui::{gtk, Appearance, OutcomeView};
use std::time::Duration;

fn labels(root: &gtk::Widget) -> Vec<String> {
    let mut pending = vec![root.clone()];
    let mut result = Vec::new();
    while let Some(widget) = pending.pop() {
        if widget.is_mapped() {
            if let Some(label) = widget.downcast_ref::<gtk::Label>() {
                result.push(label.text().to_string());
            }
        }
        let mut child = widget.first_child();
        while let Some(widget) = child {
            child = widget.next_sibling();
            pending.push(widget);
        }
    }
    result
}

fn main() {
    let dark = std::env::args().any(|arg| arg == "--dark");
    let app = gtk::Application::builder()
        .application_id("com.sevensixteen.OutcomeUpdates")
        .build();
    app.connect_activate(move |app| {
        seven_sixteen_ui::install_theme(&gdk::Display::default().unwrap(), if dark {Appearance::Dark} else {Appearance::Light}, 1., true);
        let panel = seven_sixteen_ui::column(16);
        panel.add_css_class("seven-ui");
        let outcomes = [OutcomeView::new("Completion", "running λ", false).unwrap(), OutcomeView::new("Failure", "old failure λ", true).unwrap()];
        for outcome in &outcomes {panel.append(&outcome.widget);}
        let window = gtk::ApplicationWindow::builder().application(app).child(&panel).default_width(500).default_height(400).build();
        window.present();
        let quit = app.clone();
        glib::MainContext::default().spawn_local(async move {
            glib::timeout_future(Duration::from_millis(300)).await;
            for outcome in &outcomes {
                let before = outcome.message.text();
                assert!(outcome.message.grab_focus());
                outcome.message.select_region(0, -1);
                assert!(!outcome.reconcile("Latest outcome", "failed 日本語").unwrap());
                glib::timeout_future(Duration::from_millis(100)).await;
                let visible = labels(outcome.widget.upcast_ref());
                println!("{}", serde_json::json!({"visible":visible,"selected":outcome.message.text().to_string()}));
                assert_eq!(outcome.message.text(), before);
                assert!(outcome.message.selection_bounds().is_some());
                assert!(visible.contains(&"Current value:\nLatest outcome\nfailed 日本語".into()), "Selected outcome hides its changed live value");
                assert_eq!(outcome.current.accessible_role(), gtk::AccessibleRole::Status);
                assert!(!outcome.current.is_selectable());
                let current = outcome.current.text();
                assert!(outcome.reconcile("", "invalid").is_err());
                assert_eq!(outcome.current.text(), current);
                assert!(!outcome.reconcile("Latest outcome", "stopped Ελληνικά").unwrap());
                assert_eq!(outcome.current.text(), "Current value:\nLatest outcome\nstopped Ελληνικά");
                assert!(outcome.message.selection_bounds().is_some());
                outcome.message.select_region(0, 0);
                assert!(outcome.reconcile("Latest outcome", "stopped Ελληνικά").unwrap());
                assert_eq!(outcome.message.text(), "stopped Ελληνικά");
                assert_eq!(outcome.heading.text(), "Latest outcome");
                glib::timeout_future(Duration::from_millis(100)).await;
                let visible = labels(outcome.widget.upcast_ref());
                assert!(!visible.iter().any(|text|text.starts_with("Current value:")), "Resolved selection retained the update notice");
                assert!(visible.contains(&"stopped Ελληνικά".into()));
            }
            println!("PASS: selected Result/Error text preserved with visible current value, invalid update rejection and deselection reconciliation");
            window.destroy();
            quit.quit();
        });
    });
    app.run_with_args::<&str>(&[]);
}
