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

    // GTK 4.14 added explicit announcements. Resolve the optional symbol from
    // the loaded toolkit so GTK 4.12 consumers retain their existing ABI floor.
    #[cfg(unix)]
    fn announce_valid(control: &gtk::Widget) {
        use gtk::glib::translate::ToGlibPtr;
        use std::sync::OnceLock;
        type Announce = unsafe extern "C" fn(
            *mut gtk::ffi::GtkAccessible,
            *const std::ffi::c_char,
            gtk::ffi::GtkAccessibleAnnouncementPriority,
        );
        static PROCESS: OnceLock<libloading::os::unix::Library> = OnceLock::new();
        let process = PROCESS.get_or_init(libloading::os::unix::Library::this);
        // The symbol has GTK's public signature; the widget owns the accessible
        // object and the static C string remains alive throughout the call.
        unsafe {
            if let Ok(announce) = process.get::<Announce>(b"gtk_accessible_announce\0") {
                announce(control.upcast_ref::<gtk::Accessible>().to_glib_none().0,
                         c"Entry valid.".as_ptr(), gtk::ffi::GTK_ACCESSIBLE_ANNOUNCEMENT_PRIORITY_LOW);
            }
        }
    }
    #[cfg(not(unix))]
    fn announce_valid(_control: &gtk::Widget) {}

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
                if self.control.has_focus() || self.control.focus_child().is_some_and(|child| child.has_focus()) {
                    announce_valid(&self.control);
                }
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
