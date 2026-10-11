//! Native icon adapters. Drawing coordinates are adapted from Lucide 0.474.0
//! (ISC); see reference/LICENSE.lucide and THIRD_PARTY.md. Original GTK adapter
//! code is Apache-2.0. No icon name is resolved through the host icon theme.
use crate::{button_sized, gtk, ButtonSize, ButtonVariant};
use gtk::{prelude::*, AccessibleRole};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Plus,
    ArrowRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconPosition {
    Leading,
    Trailing,
}

pub(crate) fn css(scale: f64) -> String {
    let mut css = format!(
        ".seven-ui .seven-icon {{min-width:{}px;min-height:{}px;}}",
        16.0 * scale,
        16.0 * scale
    );
    for (class, height, padding, gap, icon_size) in [
        ("", 36.0, 12.0, 8.0, 16.0),
        (".seven-button-xs", 24.0, 6.0, 4.0, 12.0),
        (".seven-button-sm", 32.0, 10.0, 6.0, 16.0),
        (".seven-button-lg", 40.0, 16.0, 8.0, 16.0),
    ] {
        css.push_str(&format!("\n.seven-ui button.seven-has-icon{class} {{padding-left:{padding}px;padding-right:{padding}px;}}\n.seven-ui button{class} .seven-button-content {{border-spacing:{gap}px;}}\n.seven-ui button{class} .seven-icon {{min-width:{icon_size}px;min-height:{icon_size}px;}}\n.seven-ui button.seven-icon-only{class} {{min-width:{height}px;min-height:{height}px;padding:0;}}\n.seven-ui button.outline.seven-icon-only{class} {{min-width:{outline}px;min-height:{outline}px;padding:0;}}",padding=padding*scale,gap=gap*scale,icon_size=icon_size*scale,height=height*scale,outline=height*scale-2.0));
    }
    css
}

fn valid_label(label: &str) -> Result<(), &'static str> {
    if label.trim().is_empty() || label.len() > 256 || label.chars().any(char::is_control) {
        Err("Icon buttons require a nonempty accessible label of at most 256 UTF-8 bytes")
    } else {
        Ok(())
    }
}

fn drawing(icon: Icon) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::builder()
        .accessible_role(AccessibleRole::Presentation)
        .build();
    area.add_css_class("seven-icon");
    area.set_halign(gtk::Align::Center);
    area.set_valign(gtk::Align::Center);
    area.set_can_target(false);
    area.set_draw_func(move |area, context, width, height| {
        let color = area.style_context().color();
        context.save().unwrap();
        context.scale(width as f64 / 24.0, height as f64 / 24.0);
        context.set_source_rgba(
            color.red() as f64,
            color.green() as f64,
            color.blue() as f64,
            color.alpha() as f64,
        );
        context.set_line_width(2.0);
        context.set_line_cap(gtk::cairo::LineCap::Round);
        context.set_line_join(gtk::cairo::LineJoin::Round);
        // Lucide plus: M5 12h14 / M12 5v14.
        // Lucide arrow-right: M5 12h14 / m12 5 7 7-7 7.
        context.move_to(5.0, 12.0);
        context.line_to(19.0, 12.0);
        context.move_to(12.0, 5.0);
        match icon {
            Icon::Plus => context.line_to(12.0, 19.0),
            Icon::ArrowRight => {
                context.line_to(19.0, 12.0);
                context.line_to(12.0, 19.0);
            }
        }
        context.stroke().unwrap();
        context.restore().unwrap();
    });
    area
}

/// A square action button with a required accessible name and native tooltip.
pub fn icon_button(
    label: &str,
    icon: Icon,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
) -> Result<gtk::Button, &'static str> {
    valid_label(label)?;
    let button = button_sized("", variant, size, disabled);
    button.add_css_class("seven-icon-only");
    button.set_child(Some(&drawing(icon)));
    // GtkButton::with_label installs LabelledBy on its original child. Clear
    // that relation after replacing/reparenting the child so a stale empty
    // label cannot override the adapter's explicit accessible name.
    button.reset_relation(gtk::AccessibleRelation::LabelledBy);
    button.update_property(&[gtk::accessible::Property::Label(label)]);
    button.set_tooltip_text(Some(label));
    Ok(button)
}

/// A literal text action with one leading or trailing icon. Activation remains
/// the host-owned native clicked event; icons have no independent action.
pub fn button_with_icon(
    label: &str,
    icon: Icon,
    position: IconPosition,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
) -> Result<gtk::Button, &'static str> {
    valid_label(label)?;
    let button = button_sized(label, variant, size, disabled);
    let text = button.child().unwrap();
    button.set_child(gtk::Widget::NONE);
    let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    content.add_css_class("seven-button-content");
    content.set_halign(gtk::Align::Center);
    content.set_valign(gtk::Align::Center);
    let icon = drawing(icon);
    match position {
        IconPosition::Leading => {
            content.append(&icon);
            content.append(&text);
        }
        IconPosition::Trailing => {
            content.append(&text);
            content.append(&icon);
        }
    }
    button.add_css_class("seven-has-icon");
    button.set_child(Some(&content));
    // GtkButton::with_label installs LabelledBy on its original child. Clear
    // that relation after replacing/reparenting the child so a stale empty
    // label cannot override the adapter's explicit accessible name.
    button.reset_relation(gtk::AccessibleRelation::LabelledBy);
    button.update_property(&[gtk::accessible::Property::Label(label)]);
    Ok(button)
}
