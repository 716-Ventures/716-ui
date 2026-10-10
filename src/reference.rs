//! Navigation intent only. The host resolves identity, scope and availability.
use gtk::prelude::*;
pub struct ReferenceView {
    pub widget: gtk::Button,
}
impl ReferenceView {
    pub fn new(label: &str) -> Result<Self, String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded accessible reference label is required".into());
        }
        let widget = crate::button(label, crate::ButtonVariant::Outline, false);
        Ok(Self { widget })
    }
    pub fn on_open(&self, f: impl Fn() + 'static) {
        self.widget.connect_clicked(move |_| f());
    }
    pub fn reconcile(&self, label: &str, available: bool) -> Result<(), String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded accessible reference label is required".into());
        }
        self.widget.set_label(label);
        self.widget.set_sensitive(available);
        self.widget.set_tooltip_text(if available {
            None
        } else {
            Some("Referenced view is unavailable")
        });
        Ok(())
    }
}
