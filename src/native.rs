//! Deterministic native controls. Events carry user data, never application authority.
pub use gtk;
use gtk::{gdk, prelude::*};

thread_local! {static THEMES:std::cell::RefCell<Vec<(gdk::Display,gtk::CssProvider)>>=const {std::cell::RefCell::new(Vec::new())};}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}
#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    Changed(String),
    Submitted(String),
}
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub background: &'static str,
    pub foreground: &'static str,
    pub muted: &'static str,
    pub border: &'static str,
    pub primary: &'static str,
    pub on_primary: &'static str,
}
impl Appearance {
    pub fn tokens(self) -> Tokens {
        match self {
            Self::Light => Tokens {
                background: "#ffffff",
                foreground: "#09090b",
                muted: "#71717a",
                border: "#71717a",
                primary: "#18181b",
                on_primary: "#fafafa",
            },
            Self::Dark => Tokens {
                background: "#09090b",
                foreground: "#fafafa",
                muted: "#a1a1aa",
                border: "#71717a",
                primary: "#fafafa",
                on_primary: "#18181b",
            },
        }
    }
}

pub fn install_theme(
    display: &gdk::Display,
    appearance: Appearance,
    text_scale: f64,
    reduced_motion: bool,
) {
    let t = appearance.tokens();
    let scale = if text_scale.is_finite() {
        text_scale.clamp(1.0, 3.0)
    } else {
        1.0
    };
    let secondary = if appearance == Appearance::Light {
        "#f4f4f5"
    } else {
        "#27272a"
    };
    let selected = if appearance == Appearance::Light {
        "#18181b"
    } else {
        "#fafafa"
    };
    let on_selected = if appearance == Appearance::Light {
        "#fafafa"
    } else {
        "#18181b"
    };
    let css = format!(
        r#"
.seven-ui {{background:{bg};color:{fg};font-family:Inter,sans-serif;font-size:{font}px;}}
.seven-ui label {{color:{fg};}}
.seven-ui label.heading {{font-size:{heading}px;font-weight:600;}}
.seven-ui label.muted {{color:{muted};}}
.seven-ui button {{border-radius:6px;min-height:{button_height}px;padding:{pad_y}px {pad_x}px;border:0 solid transparent;font-weight:500;box-shadow:none;transition:background-color {motion}ms cubic-bezier(.4,0,.2,1);}}
.seven-ui button.primary {{background:{primary};color:{on_primary};}}
.seven-ui button.secondary {{background:{secondary};color:{fg};}}
.seven-ui button.outline {{background:{outline_bg};color:{fg};min-height:{outline_height}px;border:1px solid {border};}}
.seven-ui button.ghost {{background:transparent;color:{fg};}}
.seven-ui button.destructive {{background:#b91c1c;color:#ffffff;}}
.seven-ui button.primary:hover {{background:alpha({primary},.9);}}
.seven-ui button.secondary:hover {{background:alpha({secondary},.8);}}
.seven-ui button.outline:hover {{background:{accent_hover};}}
.seven-ui button.ghost:hover {{background:{accent_hover};}}
.seven-ui button.destructive:hover {{background:alpha(#b91c1c,.9);}}
.seven-ui button:disabled {{opacity:.5;}}
.seven-ui button label {{color:inherit;}}
.seven-ui button:focus-visible,.seven-ui entry:focus-within,.seven-ui textview:focus-visible {{outline:3px solid {muted};outline-offset:2px;}}
.seven-ui entry,.seven-ui textview {{background:{bg};color:{fg};border:1px solid {border};border-radius:6px;min-height:{height}px;padding:{pad_y}px {input_x}px;box-shadow:none;caret-color:{fg};}}
.seven-ui entry {{background:{outline_bg};}}
.seven-ui entry:disabled {{opacity:.5;}}
.seven-ui entry text placeholder {{color:{muted};opacity:1;}}
.seven-ui selection {{background:{selected};color:{on_selected};}}
.seven-ui progressbar {{color:{fg};}}
.seven-ui progressbar text {{color:{fg};}}
.seven-ui button.link {{background:transparent;color:{link};border:none;}}
.seven-ui progressbar trough {{min-height:8px;background:{secondary};border-radius:6px;}}
.seven-ui progressbar progress {{min-width:2px;min-height:8px;background:{primary};border-radius:6px;border:none;box-shadow:none;}}
.seven-ui notebook.seven-tabs {{background:transparent;border:0;box-shadow:none;}}
.seven-ui notebook.seven-tabs > header {{background:{secondary};background-image:none;border:0;box-shadow:none;border-radius:8px;padding:{tabs_pad}px;min-height:{tabs_strip}px;}}
.seven-ui notebook.seven-tabs > header tabs {{background:transparent;border:0;min-height:{tabs_strip}px;}}
.seven-ui notebook.seven-tabs > header tabs tab {{background:transparent;background-image:none;border:1px solid transparent;border-radius:6px;box-shadow:none;margin:0 0 {tabs_y}px 0;padding:{tabs_y}px {tabs_x}px;min-width:0;min-height:{tabs_height}px;color:{tabs_inactive};}}
.seven-ui notebook.seven-tabs > header tabs tab label {{color:inherit;font-weight:500;}}
.seven-ui notebook.seven-tabs > header tabs tab:hover {{color:{fg};}}
.seven-ui notebook.seven-tabs > header tabs tab:checked {{background:{outline_bg};border-color:{tabs_border};color:{fg};}}
.seven-ui notebook.seven-tabs > stack {{background:transparent;border:0;margin-top:{tabs_gap}px;}}
"#,
        tabs_pad = 3.0 * scale,
        tabs_strip = 30.0 * scale,
        tabs_x = 8.0 * scale,
        tabs_y = 4.0 * scale,
        tabs_height = 19.0 * scale,
        tabs_gap = 8.0 * scale,
        tabs_inactive = if appearance == Appearance::Light { "alpha(#09090b,.6)" } else { t.muted },
        tabs_border = if appearance == Appearance::Light { "transparent" } else { secondary },
        outline_bg = if appearance == Appearance::Light {
            t.background
        } else {
            "alpha(#27272a,.3)"
        },
        accent_hover = if appearance == Appearance::Light {
            secondary
        } else {
            "alpha(#27272a,.5)"
        },
        bg = t.background,
        fg = t.foreground,
        muted = t.muted,
        border = t.border,
        primary = t.primary,
        on_primary = t.on_primary,
        font = 14.0 * scale,
        heading = 20.0 * scale,
        height = 18.0 * scale,
        button_height = 20.0 * scale,
        outline_height = 20.0 * scale - 2.0,
        pad_y = 8.0 * scale,
        pad_x = 16.0 * scale,
        input_x = 12.0 * scale,
        motion = if reduced_motion { 0 } else { 150 },
        link = if appearance == Appearance::Light {
            "#0e7490"
        } else {
            "#67e8f9"
        }
    );
    let provider = gtk::CssProvider::new();
    provider.load_from_string(&css);
    THEMES.with(|themes| {
        let mut themes = themes.borrow_mut();
        themes.retain(|(target, old)| {
            if target == display {
                gtk::style_context_remove_provider_for_display(display, old);
                false
            } else {
                true
            }
        });
        themes.push((display.clone(), provider.clone()));
    });
    gtk::style_context_add_provider_for_display(
        display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    {
        let settings = gtk::Settings::for_display(display);
        settings.set_gtk_enable_animations(!reduced_motion);
        settings.set_gtk_application_prefer_dark_theme(appearance == Appearance::Dark);
    }
}

pub fn column(spacing: i32) -> gtk::Box {
    gtk::Box::new(gtk::Orientation::Vertical, spacing.clamp(0, 64))
}
pub fn row(spacing: i32) -> gtk::Box {
    gtk::Box::new(gtk::Orientation::Horizontal, spacing.clamp(0, 64))
}
pub fn text(value: &str, heading: bool) -> gtk::Label {
    let label = gtk::Label::builder()
        .label(value)
        .accessible_role(if heading {
            gtk::AccessibleRole::Heading
        } else {
            gtk::AccessibleRole::Label
        })
        .build();
    if heading {
        label.update_property(&[gtk::accessible::Property::Level(1)]);
    }
    label.set_selectable(true);
    label.set_wrap(true);
    label.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    label.set_xalign(0.0);
    label.set_natural_wrap_mode(gtk::NaturalWrapMode::Word);
    label.set_hexpand(true);
    if heading {
        label.add_css_class("heading");
    }
    label
}
pub fn button(label: &str, variant: ButtonVariant, disabled: bool) -> gtk::Button {
    let button = gtk::Button::with_label(label);
    button.add_css_class(match variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Outline => "outline",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Destructive => "destructive",
    });
    button.set_sensitive(!disabled);
    button
}
pub fn link(label: &str, url: &str) -> Result<gtk::LinkButton, &'static str> {
    if !(url.starts_with("https://") || url.starts_with("http://") || url.starts_with("mailto:"))
        || url.chars().any(char::is_control)
    {
        return Err("Unsupported link scheme");
    }
    let link = gtk::LinkButton::with_label(url, label);
    link.update_property(&[gtk::accessible::Property::Label(label)]);
    Ok(link)
}
pub fn status(value: &str) -> gtk::Label {
    let label = text(value, false);
    label.add_css_class("muted");
    label.set_property("accessible-role", gtk::AccessibleRole::Status);
    label
}
pub fn progress(label: &str, value: Option<f64>) -> gtk::ProgressBar {
    let progress = gtk::ProgressBar::new();
    progress.set_show_text(true);
    progress.set_text(Some(label));
    progress.update_property(&[gtk::accessible::Property::Label(label)]);
    if let Some(value) = value.filter(|v| v.is_finite()) {
        progress.set_fraction(value.clamp(0.0, 1.0));
    } else {
        progress.pulse();
    }
    progress
}
#[derive(Clone)]
pub struct TextField {
    pub widget: gtk::Box,
    pub entry: gtk::Entry,
}
impl TextField {
    pub fn new(label: &str, placeholder: &str, value: &str) -> Self {
        let widget = column(6);
        let name = text(label, false);
        name.set_selectable(false);
        widget.append(&name);
        let entry = gtk::Entry::new();
        entry.set_placeholder_text(Some(placeholder));
        entry.set_text(value);
        entry.set_hexpand(true);
        entry.update_property(&[gtk::accessible::Property::Label(label)]);
        widget.append(&entry);
        Self { widget, entry }
    }
    pub fn on_event(&self, callback: impl Fn(InputEvent) + 'static) {
        let callback = std::rc::Rc::new(callback);
        let changed = callback.clone();
        self.entry
            .connect_changed(move |entry| changed(InputEvent::Changed(entry.text().to_string())));
        self.entry.connect_activate(move |entry| {
            callback(InputEvent::Submitted(entry.text().to_string()))
        });
    }
    /// Domain updates must never replace an actively edited draft.
    pub fn reconcile(&self, value: &str) -> bool {
        if self.entry.has_focus() || self.entry.focus_child().is_some() {
            return false;
        }
        if self.entry.text() != value {
            self.entry.set_text(value);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn luminance(hex: &str) -> f64 {
        let channels = (0..3)
            .map(|i| u8::from_str_radix(&hex[1 + i * 2..3 + i * 2], 16).unwrap() as f64 / 255.0)
            .map(|v| {
                if v <= 0.04045 {
                    v / 12.92
                } else {
                    ((v + 0.055) / 1.055).powf(2.4)
                }
            })
            .collect::<Vec<_>>();
        channels[0] * 0.2126 + channels[1] * 0.7152 + channels[2] * 0.0722
    }
    fn contrast(a: &str, b: &str) -> f64 {
        let (a, b) = (luminance(a), luminance(b));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }
    #[test]
    fn reading_tokens_pass_both_appearances() {
        for appearance in [Appearance::Light, Appearance::Dark] {
            let t = appearance.tokens();
            assert!(contrast(t.foreground, t.background) >= 4.5);
            assert!(contrast(t.muted, t.background) >= 4.5);
            assert!(contrast(t.primary, t.on_primary) >= 4.5);
            assert!(contrast(t.border, t.background) >= 3.0);
        }
    }
}

/// Multiline editing uses GTK's native buffer, selection, IME and local undo.
#[derive(Clone)]
pub struct TextArea {
    pub widget: gtk::Box,
    pub view: gtk::TextView,
}
impl TextArea {
    pub fn new(label: &str, value: &str) -> Self {
        let widget = column(6);
        let name = text(label, false);
        name.set_selectable(false);
        widget.append(&name);
        let view = gtk::TextView::new();
        view.set_wrap_mode(gtk::WrapMode::WordChar);
        view.set_hexpand(true);
        view.set_height_request(100);
        view.buffer().set_enable_undo(true);
        view.buffer().set_text(value);
        view.update_property(&[gtk::accessible::Property::Label(label)]);
        widget.append(&view);
        Self { widget, view }
    }
    pub fn value(&self) -> String {
        let buffer = self.view.buffer();
        buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .into()
    }
    pub fn reconcile(&self, value: &str) -> bool {
        if self.view.has_focus() || self.view.is_focus() {
            return false;
        }
        if self.value() != value {
            self.view.buffer().set_text(value);
        }
        true
    }
    pub fn on_event(&self, callback: impl Fn(InputEvent) + 'static) {
        let callback = std::rc::Rc::new(callback);
        let changed = callback.clone();
        self.view.buffer().connect_changed(move |buffer| {
            changed(InputEvent::Changed(
                buffer
                    .text(&buffer.start_iter(), &buffer.end_iter(), false)
                    .into(),
            ))
        });
        let view = self.view.clone();
        let keys = gtk::EventControllerKey::new();
        keys.connect_key_pressed(move |_, key, _, modifiers| {
            if key == gdk::Key::Return && modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                let buffer = view.buffer();
                callback(InputEvent::Submitted(
                    buffer
                        .text(&buffer.start_iter(), &buffer.end_iter(), false)
                        .into(),
                ));
                gtk::glib::Propagation::Stop
            } else {
                gtk::glib::Propagation::Proceed
            }
        });
        self.view.add_controller(keys);
    }
}
