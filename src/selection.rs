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
    pub validation: crate::ValidationFeedback,
    pub heading: gtk::Label,
    title: std::rc::Rc<std::cell::RefCell<String>>,
    name: std::rc::Rc<dyn Fn(&gtk::DropDown)>,
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
        let heading = crate::text(title, false);
        widget.append(&heading);
        let strings = options.iter().map(String::as_str).collect::<Vec<_>>();
        let control = gtk::DropDown::from_strings(&strings);
        control.add_css_class("seven-choice");
        control.set_selected(selected as u32);
        control.update_property(&[gtk::accessible::Property::Label(title)]);
        // The nested GTK trigger can otherwise replace the authored name
        // with only its selected label. Keep both field identity and value.
        let title = std::rc::Rc::new(std::cell::RefCell::new(title.to_owned()));
        let current_title = title.clone();
        let names = options.to_vec();
        let name: std::rc::Rc<dyn Fn(&gtk::DropDown)> = std::rc::Rc::new(move |control: &gtk::DropDown| {
            let value = names.get(control.selected() as usize).map(String::as_str).unwrap_or("No option selected");
            control.reset_relation(gtk::AccessibleRelation::LabelledBy);
            control.update_property(&[gtk::accessible::Property::Label(&format!("{}: {value}",current_title.borrow().as_str()))]);
        });
        name(&control);
        let mapped = name.clone();control.connect_map(move |control| mapped(control));
        let selected_name = name.clone();
        control.connect_selected_notify(move |control| selected_name(control));
        widget.append(&control);
        let validation = crate::ValidationFeedback::new(&control);
        widget.append(&validation.label);
        Ok(Self {
            widget,
            control,
            validation,
            heading,
            title,
            name,
            options: options.to_vec(),
        })
    }
    /// Keep the visible field identity and accessible name together. A selected
    /// heading remains stable until the host retries after selection/focus ends.
    pub fn set_label(&self, title: &str) -> Result<bool, String> {
        label(title)?;
        if self.title.borrow().as_str() == title { return Ok(true); }
        if self.heading.has_focus() || self.heading.selection_bounds().is_some() { return Ok(false); }
        *self.title.borrow_mut() = title.to_owned();
        self.heading.set_text(title);
        (self.name)(&self.control);
        Ok(true)
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
    /// Explain an invalid local value without changing it or emitting an event.
    pub fn set_validation_message(&self, message: Option<&str>) -> Result<(), &'static str> {
        self.validation.set_message(message)
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
    pub validation: crate::ValidationFeedback,
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
        let validation = crate::ValidationFeedback::new(&control);
        widget.append(&validation.label);
        Ok(Self { widget, control, validation })
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
    /// Explain an invalid local value without changing it or emitting an event.
    pub fn set_validation_message(&self, message: Option<&str>) -> Result<(), &'static str> {
        self.validation.set_message(message)
    }
    pub fn on_changed(&self, f: impl Fn(bool) + 'static) {
        self.control
            .connect_toggled(move |control| f(control.is_active()));
    }
}
