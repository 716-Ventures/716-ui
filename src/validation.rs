//! Literal validation feedback belongs to the host; it never changes a draft.
pub const MAX_VALIDATION_MESSAGE_BYTES: usize = 1024;

pub fn validate_validation_message(message: &str) -> Result<(), &'static str> {
    if message.trim().is_empty()
        || message.len() > MAX_VALIDATION_MESSAGE_BYTES
        || message.chars().any(|c| c.is_control() && c != '\n' && c != '\t')
    {
        return Err("Validation messages require 1–1024 UTF-8 bytes of readable text");
    }
    Ok(())
}

#[cfg(feature = "native")]
mod native {
    use crate::gtk::{self, prelude::*};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Clone)]
    pub struct ValidationFeedback {
        pub label: gtk::Label,
        control: gtk::Widget,
        message: Rc<RefCell<Option<String>>>,
    }
    impl ValidationFeedback {
        pub fn new(control: &impl IsA<gtk::Widget>) -> Self {
            let label = gtk::Label::builder()
                .accessible_role(gtk::AccessibleRole::Alert)
                .wrap(true)
                .wrap_mode(gtk::pango::WrapMode::WordChar)
                .xalign(0.0)
                .selectable(true)
                .visible(false)
                .build();
            label.add_css_class("seven-validation-error");
            Self { label, control: control.as_ref().clone(), message: Rc::default() }
        }
        fn error_relation() -> gtk::AccessibleRelation {
            use gtk::glib::{self, translate::Uninitialized};
            // Immediately initialize the zeroed GValue with GTK’s relation type.
            let mut expected = unsafe { glib::Value::uninitialized() };
            gtk::AccessibleRelation::ErrorMessage.init_value(&mut expected);
            if expected.type_() == gtk::Accessible::static_type() {
                // GTK 4.14 accepts one object but its AT-SPI bridge reads a list.
                // Use its working description relation until that upstream fix.
                gtk::AccessibleRelation::DescribedBy
            } else {
                gtk::AccessibleRelation::ErrorMessage
            }
        }
        fn relate_error(&self) {
            let targets = [self.label.upcast_ref()];
            let relation = if Self::error_relation() == gtk::AccessibleRelation::DescribedBy {
                gtk::accessible::Relation::DescribedBy(&targets)
            } else {
                gtk::accessible::Relation::ErrorMessage(&targets)
            };
            self.control.update_relation(&[relation]);
        }
        /// Reject invalid feedback before changing any visible/accessible state.
        pub fn set_message(&self, message: Option<&str>) -> Result<(), &'static str> {
            if let Some(message) = message { super::validate_validation_message(message)?; }
            if self.message.borrow().as_deref() == message { return Ok(()); }
            if let Some(message) = message {
                self.label.set_label(message);
                self.label.set_visible(true);
                self.control.add_css_class("seven-invalid");
                self.control.update_state(&[gtk::accessible::State::Invalid(gtk::AccessibleInvalidState::True)]);
                self.relate_error();
            } else {
                self.control.reset_relation(Self::error_relation());
                self.control.update_state(&[gtk::accessible::State::Invalid(gtk::AccessibleInvalidState::False)]);
                self.control.remove_css_class("seven-invalid");
                self.label.set_visible(false);
                self.label.set_label("");
            }
            *self.message.borrow_mut() = message.map(str::to_owned);
            Ok(())
        }
    }
}
#[cfg(feature = "native")]
pub use native::ValidationFeedback;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn feedback_is_bounded_readable_literal_unicode() {
        for message in ["Choose a saved workspace.", "日本語 λ\nTry again.", "<b>Literal</b> & text", &"λ".repeat(512)] {
            assert!(validate_validation_message(message).is_ok());
        }
        for message in ["", " \n\t", "bad\0text", "bad\x1btext", &"λ".repeat(513)] {
            assert!(validate_validation_message(message).is_err());
        }
    }
}
