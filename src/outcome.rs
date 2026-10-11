//! Readable work outcomes; recovery emits local intent for the host to assess.
use gtk::prelude::*;
pub struct OutcomeView {
    pub widget: gtk::Box,
    pub heading: gtk::Label,
    pub message: gtk::Label,
    pub current: gtk::Label,
    pub recovery: gtk::Button,
    observed: std::cell::RefCell<(String, String)>,
    error: bool,
}
fn title(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 256 && !value.contains('\0')
}
impl OutcomeView {
    pub fn new(label: &str, message: &str, error: bool) -> Result<Self, String> {
        if !title(label) || message.len() > 65536 || message.contains('\0') {
            return Err("A bounded outcome label and literal message are required".into());
        }
        let widget = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .accessible_role(if error {
                gtk::AccessibleRole::Alert
            } else {
                gtk::AccessibleRole::Status
            })
            .build();
        let observed = std::cell::RefCell::new((label.to_owned(), message.to_owned()));
        let heading = crate::text(label, true);
        let message = crate::text(message, false);
        let current = crate::status("");
        current.set_selectable(false);
        current.set_visible(false);
        let recovery = crate::button("Try again", crate::ButtonVariant::Outline, true);
        recovery.set_visible(false);
        widget.append(&heading);
        widget.append(&message);
        widget.append(&current);
        widget.append(&recovery);
        Ok(Self {
            widget,
            heading,
            message,
            current,
            recovery,
            observed,
            error,
        })
    }
    pub fn reconcile(&self, label: &str, message: &str) -> Result<bool, String> {
        if !title(label) || message.len() > 65536 || message.contains('\0') {
            return Err("A bounded outcome label and literal message are required".into());
        }
        let changed = {
            let previous = self.observed.borrow();
            previous.0 != label || previous.1 != message
        };
        if changed {
            *self.observed.borrow_mut() = (label.to_owned(), message.to_owned());
            // Announce the actual observation even while a selected snapshot
            // stays visible. Bound speech; complete literal text remains readable.
            let mut announcement: String = format!("{label}: {message}").chars().take(512).collect();
            if label.chars().count() + 2 + message.chars().count() > 512 { announcement.push('…'); }
            crate::announcement::announce(&self.widget, &announcement,
                if self.error { gtk::ffi::GTK_ACCESSIBLE_ANNOUNCEMENT_PRIORITY_HIGH }
                else { gtk::ffi::GTK_ACCESSIBLE_ANNOUNCEMENT_PRIORITY_LOW });
        }
        if self.heading.selection_bounds().is_some() || self.message.selection_bounds().is_some() {
            // Selection retains its literal snapshot, not authority to freeze
            // the observed work state. Keep current source text visible too.
            let changed = self.heading.text() != label || self.message.text() != message;
            if changed {
                let value = format!("Current value:\n{label}\n{message}");
                if self.current.text() != value {
                    self.current.set_text(&value);
                }
            }
            self.current.set_visible(changed);
            return Ok(false);
        }
        self.current.set_visible(false);
        self.heading.set_text(label);
        self.message.set_text(message);
        Ok(true)
    }
    pub fn set_recovery(&self, label: &str, available: bool) -> Result<(), String> {
        if !title(label) {
            return Err("A bounded recovery label is required".into());
        }
        self.recovery.set_label(label);
        self.recovery.set_visible(true);
        self.recovery.set_sensitive(available);
        Ok(())
    }
    pub fn on_recover(&self, f: impl Fn() + 'static) {
        self.recovery.connect_clicked(move |_| f());
    }
}
