//! Native images consume bounded bytes supplied by the host, never file paths or URLs.
use gtk::prelude::*;
pub struct ImageView {
    pub widget: gtk::Box,
    pub picture: gtk::Picture,
    pub caption: gtk::Label,
}
impl ImageView {
    pub fn new(label: &str, bytes: &[u8]) -> Result<Self, String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded alternative label is required".into());
        }
        let dimensions = crate::content::png_dimensions(bytes)?;
        let texture = gtk::gdk::Texture::from_bytes(&gtk::glib::Bytes::from(bytes))
            .map_err(|_| "Native image decoding failed")?;
        if texture.width() as u32 != dimensions.0 || texture.height() as u32 != dimensions.1 {
            return Err("Decoded image dimensions differ".into());
        }
        let widget = crate::column(8);
        let picture = gtk::Picture::for_paintable(&texture);
        picture.set_can_shrink(true);
        picture.set_hexpand(true);
        picture.set_vexpand(true);
        picture.set_content_fit(gtk::ContentFit::Contain);
        picture.update_property(&[gtk::accessible::Property::Label(label)]);
        let viewport = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .min_content_width(160)
            .min_content_height(240)
            .max_content_height(240)
            .propagate_natural_height(true)
            .child(&picture)
            .build();
        widget.append(&viewport);
        let caption = crate::text(label, false);
        widget.append(&caption);
        Ok(Self {
            widget,
            picture,
            caption,
        })
    }
    pub fn set_label(&self, label: &str) -> Result<(), String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded alternative label is required".into());
        }
        self.caption.set_text(label);
        self.picture
            .update_property(&[gtk::accessible::Property::Label(label)]);
        Ok(())
    }
}
