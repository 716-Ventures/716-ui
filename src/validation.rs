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
        fn relate_error(&self) {
            use gtk::glib::{self, translate::*};
            let relation = gtk::AccessibleRelation::ErrorMessage;
            // Initialize the zeroed GValue immediately with GTK’s relation type.
            let mut expected = unsafe { glib::Value::uninitialized() };
            relation.init_value(&mut expected);
            // GTK 4.14 stores errormessage as one object; 4.16+ stores a
            // reference list. gtk-rs 0.9 models only the latter. Ask the loaded
            // GTK for its value type rather than guessing from package versions.
            if expected.type_().is_a(glib::Object::static_type()) {
                let value = self.label.upcast_ref::<gtk::Accessible>().to_value();
                let mut key = relation.into_glib();
                // Both one-element arrays and their referenced objects remain
                // alive during this synchronous GTK call, which copies the value.
                unsafe {
                    gtk::ffi::gtk_accessible_update_relation_value(
                        self.control.upcast_ref::<gtk::Accessible>().to_glib_none().0,
                        1, &mut key, value.to_glib_none().0,
                    );
                }
            } else {
                self.control.update_relation(&[gtk::accessible::Relation::ErrorMessage(&[self.label.upcast_ref()])]);
            }
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
                self.control.reset_relation(gtk::AccessibleRelation::ErrorMessage);
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
