//! Selectable live status text with a visible current observation.
use gtk::prelude::*;

pub struct StatusView {
    pub widget: gtk::Box,
    pub value: gtk::Label,
    pub current: gtk::Label,
    observed: std::cell::RefCell<String>,
}

fn valid(value: &str) -> bool {
    value.len() <= 65536 && !value.contains('\0')
}

impl StatusView {
    pub fn new(value: &str) -> Result<Self, String> {
        if !valid(value) {
            return Err("Status requires bounded literal text".into());
        }
        let widget = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .accessible_role(gtk::AccessibleRole::Status)
            .build();
        let text = crate::text(value, false);
        text.add_css_class("muted");
        let current = crate::status("");
        current.set_selectable(false);
        current.set_visible(false);
        widget.append(&text);
        widget.append(&current);
        Ok(Self {
            widget,
            value: text,
            current,
            observed: std::cell::RefCell::new(value.to_owned()),
        })
    }

    /// Returns false while selection protects the value label. The latest
    /// observation remains visible beside it and is announced once when mapped.
    /// Invalid observations leave both representations unchanged.
    pub fn reconcile(&self, value: &str) -> Result<bool, String> {
        if !valid(value) {
            return Err("Status requires bounded literal text".into());
        }
        if self.observed.borrow().as_str() != value {
            *self.observed.borrow_mut() = value.to_owned();
            crate::announcement::announce_bounded(
                &self.widget, value,
                gtk::ffi::GTK_ACCESSIBLE_ANNOUNCEMENT_PRIORITY_LOW,
            );
        }
        if self.value.selection_bounds().is_some() {
            let changed = self.value.text() != value;
            if changed {
                let current = format!("Current value:\n{value}");
                if self.current.text() != current {
                    self.current.set_text(&current);
                }
            }
            self.current.set_visible(changed);
            return Ok(false);
        }
        self.current.set_visible(false);
        if self.value.text() != value {
            self.value.set_text(value);
        }
        Ok(true)
    }
}
