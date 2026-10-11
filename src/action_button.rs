//! Reconcile button content without replacing its focus or host action owner.
use crate::{gtk, ButtonContent, ButtonProps};
use gtk::prelude::*;
const STYLE_CLASSES: &[&str] = &[
    "primary",
    "secondary",
    "outline",
    "ghost",
    "destructive",
    "button-link",
    "seven-button-xs",
    "seven-button-sm",
    "seven-button-lg",
    "seven-icon-only",
    "seven-has-icon",
];
fn build(props: &ButtonProps) -> Result<gtk::Button, &'static str> {
    props.validate()?;
    match props.content() {
        ButtonContent::Text => Ok(crate::button_sized(
            &props.label,
            props.variant,
            props.size,
            props.disabled,
        )),
        ButtonContent::IconOnly(icon) => crate::icon_button(
            &props.label,
            icon,
            props.variant,
            props.size,
            props.disabled,
        ),
        ButtonContent::WithIcon(icon, position) => crate::button_with_icon(
            &props.label,
            icon,
            position,
            props.variant,
            props.size,
            props.disabled,
        ),
    }
}
pub struct ActionButton {
    pub widget: gtk::Button,
    props: ButtonProps,
}
impl ActionButton {
    pub fn new(props: &ButtonProps) -> Result<Self, &'static str> {
        Ok(Self {
            widget: build(props)?,
            props: props.clone(),
        })
    }
    /// Validate before mutation. The GtkButton, focus and connected host action
    /// survive content changes; identical content retains its existing children.
    pub fn reconcile(&mut self, props: &ButtonProps) -> Result<(), &'static str> {
        props.validate()?;
        if self.props == *props {
            return Ok(());
        }
        if self.props.label != props.label
            || self.props.variant != props.variant
            || self.props.size != props.size
            || self.props.content() != props.content()
        {
            let candidate = build(props)?;
            let child = candidate.child().unwrap();
            candidate.set_child(gtk::Widget::NONE);
            self.widget
                .reset_relation(gtk::AccessibleRelation::LabelledBy);
            self.widget.set_child(Some(&child));
            for class in STYLE_CLASSES {
                self.widget.remove_css_class(class);
                if candidate.has_css_class(class) {
                    self.widget.add_css_class(class);
                }
            }
            self.widget
                .update_property(&[gtk::accessible::Property::Label(&props.label)]);
            self.widget
                .set_tooltip_text(candidate.tooltip_text().as_deref());
            crate::native::settle_button_fonts(&self.widget.display());
        }
        self.widget.set_sensitive(!props.disabled);
        self.props = props.clone();
        Ok(())
    }
}
