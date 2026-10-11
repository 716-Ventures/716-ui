//! Optional GTK announcements, preserving the library's GTK 4.12 ABI floor.
use crate::gtk::{self, prelude::*};

#[cfg(unix)]
pub(crate) fn announce(widget: &impl IsA<gtk::Widget>, message: &str, priority: gtk::ffi::GtkAccessibleAnnouncementPriority) {
    let widget = widget.as_ref();
    if !widget.is_mapped() { return; }
    let Ok(message) = std::ffi::CString::new(message) else { return; };
    use gtk::glib::translate::ToGlibPtr;
    use std::sync::OnceLock;
    type Announce = unsafe extern "C" fn(
        *mut gtk::ffi::GtkAccessible,
        *const std::ffi::c_char,
        gtk::ffi::GtkAccessibleAnnouncementPriority,
    );
    static PROCESS: OnceLock<libloading::os::unix::Library> = OnceLock::new();
    let process = PROCESS.get_or_init(libloading::os::unix::Library::this);
    // The optional GTK 4.14 symbol has this public signature; the widget and
    // validated C string remain alive throughout the synchronous call.
    unsafe {
        if let Ok(announce) = process.get::<Announce>(b"gtk_accessible_announce\0") {
            announce(widget.upcast_ref::<gtk::Accessible>().to_glib_none().0,
                     message.as_ptr(), priority);
        }
    }
}

#[cfg(not(unix))]
pub(crate) fn announce(_widget: &impl IsA<gtk::Widget>, _message: &str, _priority: gtk::ffi::GtkAccessibleAnnouncementPriority) {}
