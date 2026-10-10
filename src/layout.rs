//! Semantic sections and bounded native scrolling; no application state or effects.
use gtk::prelude::*;
fn label(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 256 || value.contains('\0') {
        Err("A bounded accessible label is required".into())
    } else {
        Ok(())
    }
}
pub struct Section {
    pub widget: gtk::Box,
    pub heading: gtk::Label,
    pub content: gtk::Box,
}
impl Section {
    pub fn new(title: &str) -> Result<Self, String> {
        label(title)?;
        let widget = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .accessible_role(gtk::AccessibleRole::Group)
            .build();
        widget.update_property(&[gtk::accessible::Property::Label(title)]);
        let heading = crate::text(title, true);
        let content = crate::column(12);
        widget.append(&heading);
        widget.append(&content);
        Ok(Self {
            widget,
            heading,
            content,
        })
    }
    pub fn set_label(&self, title: &str) -> Result<(), String> {
        label(title)?;
        self.heading.set_text(title);
        self.widget
            .update_property(&[gtk::accessible::Property::Label(title)]);
        Ok(())
    }
}
pub struct ScrollRegion {
    pub widget: gtk::ScrolledWindow,
    pub content: gtk::Box,
}
impl ScrollRegion {
    pub fn new(title: &str) -> Result<Self, String> {
        label(title)?;
        let content = crate::column(12);
        let widget = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .min_content_height(160)
            .max_content_height(480)
            .propagate_natural_height(true)
            .child(&content)
            .build();
        widget.update_property(&[gtk::accessible::Property::Label(title)]);
        Ok(Self { widget, content })
    }
}
