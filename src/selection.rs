//! Local native form values; committing or dispatching them belongs to the host.
use gtk::prelude::*;
fn label(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 256 || value.contains('\0') {
        Err("A bounded accessible label is required".into())
    } else {
        Ok(())
    }
}
pub struct Choice {
    pub widget: gtk::Box,
    pub control: gtk::DropDown,
    options: Vec<String>,
}
impl Choice {
    pub fn new(title: &str, options: &[String], value: &str) -> Result<Self, String> {
        label(title)?;
        let mut unique = std::collections::BTreeSet::new();
        if options.is_empty()
            || options.len() > 32
            || options
                .iter()
                .any(|v| label(v).is_err() || !unique.insert(v))
        {
            return Err("Choice requires 1–32 distinct bounded options".into());
        }
        let selected = options
            .iter()
            .position(|v| v == value)
            .ok_or("Choice value must name an option")?;
        let widget = crate::column(6);
        widget.append(&crate::text(title, false));
        let strings = options.iter().map(String::as_str).collect::<Vec<_>>();
        let control = gtk::DropDown::from_strings(&strings);
        control.set_selected(selected as u32);
        control.update_property(&[gtk::accessible::Property::Label(title)]);
        widget.append(&control);
        Ok(Self {
            widget,
            control,
            options: options.to_vec(),
        })
    }
    pub fn value(&self) -> Option<String> {
        self.options.get(self.control.selected() as usize).cloned()
    }
    pub fn reconcile(&self, value: &str) -> bool {
        if self.control.has_focus() || self.control.focus_child().is_some() {
            return false;
        }
        if let Some(index) = self.options.iter().position(|v| v == value) {
            self.control.set_selected(index as u32);
            true
        } else {
            false
        }
    }
    pub fn options(&self) -> &[String] {
        &self.options
    }
    pub fn on_changed(&self, f: impl Fn(String) + 'static) {
        let options = self.options.clone();
        self.control.connect_selected_notify(move |control| {
            if let Some(value) = options.get(control.selected() as usize) {
                f(value.clone());
            }
        });
    }
}
pub struct Toggle {
    pub widget: gtk::Box,
    pub control: gtk::CheckButton,
}
impl Toggle {
    pub fn new(title: &str, value: bool) -> Result<Self, String> {
        label(title)?;
        let widget = crate::column(6);
        let control = gtk::CheckButton::with_label(title);
        control.add_css_class("seven-toggle");
        control.set_active(value);
        // GTK 4.14 exposes generic widget actions over AT-SPI, but does not
        // supply a CheckButton activation action. Keep this local to the
        // control and use a weak reference so the action cannot retain it.
        let actions = gtk::gio::SimpleActionGroup::new();
        let toggle = gtk::gio::SimpleAction::new("toggle", None);
        let weak_control = control.downgrade();
        toggle.connect_activate(move |_, _| {
            if let Some(control) = weak_control.upgrade() {
                if control.is_sensitive() {
                    control.emit_activate();
                }
            }
        });
        actions.add_action(&toggle);
        control.insert_action_group("control", Some(&actions));
        control.update_property(&[gtk::accessible::Property::Label(title)]);
        widget.append(&control);
        Ok(Self { widget, control })
    }
    pub fn reconcile(&self, value: bool) -> bool {
        if self.control.has_focus()
            || self.control.is_focus()
            || self.control.focus_child().is_some()
        {
            return false;
        }
        self.control.set_active(value);
        true
    }
    pub fn on_changed(&self, f: impl Fn(bool) + 'static) {
        self.control
            .connect_toggled(move |control| f(control.is_active()));
    }
}
