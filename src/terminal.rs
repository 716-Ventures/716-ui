//! Native terminal rendering only; the host owns attachment, transport and authority.
use gtk::prelude::*;
use vte::prelude::*;
pub struct TerminalView {
    pub widget: gtk::Box,
    pub terminal: vte::Terminal,
    pub heading: gtk::Label,
}
impl TerminalView {
    pub fn new(label: &str) -> Result<Self, String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded terminal label is required".into());
        }
        let widget = crate::column(8);
        let heading = crate::text(label, true);
        widget.append(&heading);
        let terminal = vte::Terminal::new();
        terminal.set_scrollback_lines(1000);
        terminal.set_input_enabled(false);
        terminal.set_size(80, 24);
        terminal.set_hexpand(true);
        terminal.set_vexpand(true);
        terminal.update_property(&[gtk::accessible::Property::Label(label)]);
        let scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .min_content_height(240)
            .max_content_height(640)
            .propagate_natural_height(true)
            .child(&terminal)
            .build();
        widget.append(&scroll);
        Ok(Self {
            widget,
            terminal,
            heading,
        })
    }
    pub fn feed(&self, bytes: &[u8], reset: bool) -> Result<(), String> {
        if bytes.len() > 16384 {
            return Err("Terminal display chunk exceeds 16 KiB".into());
        }
        if reset {
            self.terminal.reset(true, true);
        }
        self.terminal.feed(bytes);
        Ok(())
    }
    pub fn set_attached(&self, attached: bool) {
        self.terminal.set_input_enabled(attached);
    }
    pub fn input_enabled(&self) -> bool {
        self.terminal.is_input_enabled()
    }
    pub fn is_protected(&self) -> bool {
        self.terminal.has_selection()
            || self.terminal.has_focus()
            || self.terminal.is_focus()
            || self.heading.selection_bounds().is_some()
    }
    pub fn on_input(&self, f: impl Fn(Vec<u8>) + 'static) {
        self.terminal
            .connect_commit(move |_, text, _| f(text.as_bytes().to_vec()));
    }
    pub fn grid(&self) -> (u32, u32) {
        (
            self.terminal.row_count().clamp(1, 500) as u32,
            self.terminal.column_count().clamp(1, 1000) as u32,
        )
    }
}
