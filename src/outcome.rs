//! Readable work outcomes; recovery emits local intent for the host to assess.
use gtk::prelude::*;
pub struct OutcomeView {
    pub widget: gtk::Box,
    pub heading: gtk::Label,
    pub message: gtk::Label,
    pub recovery: gtk::Button,
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
        let heading = crate::text(label, true);
        let message = crate::text(message, false);
        let recovery = crate::button("Try again", crate::ButtonVariant::Outline, true);
        recovery.set_visible(false);
        widget.append(&heading);
        widget.append(&message);
        widget.append(&recovery);
        Ok(Self {
            widget,
            heading,
            message,
            recovery,
        })
    }
    pub fn reconcile(&self, label: &str, message: &str) -> Result<bool, String> {
        if !title(label) || message.len() > 65536 || message.contains('\0') {
            return Err("A bounded outcome label and literal message are required".into());
        }
        if self.heading.selection_bounds().is_some() || self.message.selection_bounds().is_some() {
            return Ok(false);
        }
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
