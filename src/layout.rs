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

/// Two native panes. Divider movement stays local and survives content updates.
pub struct SplitView {
    pub widget: gtk::Paned,
}
impl SplitView {
    pub fn new(title: &str, vertical: bool) -> Result<Self, String> {
        label(title)?;
        let widget = gtk::Paned::builder()
            .orientation(if vertical {
                gtk::Orientation::Vertical
            } else {
                gtk::Orientation::Horizontal
            })
            .wide_handle(true)
            .shrink_start_child(false)
            .shrink_end_child(false)
            .resize_start_child(true)
            .resize_end_child(true)
            .build();
        widget.update_property(&[gtk::accessible::Property::Label(title)]);
        Ok(Self { widget })
    }
    pub fn children(&self) -> Vec<gtk::Widget> {
        [self.widget.start_child(), self.widget.end_child()]
            .into_iter()
            .flatten()
            .collect()
    }
    pub fn clear(&self) {
        self.widget.set_start_child(gtk::Widget::NONE);
        self.widget.set_end_child(gtk::Widget::NONE);
    }
    pub fn set_children(&self, first: &gtk::Widget, second: &gtk::Widget) -> Result<(), String> {
        if first == second
            || [first, second].iter().any(|w| {
                w.parent()
                    .is_some_and(|p| p != self.widget.clone().upcast::<gtk::Widget>())
            })
        {
            return Err(
                "Split children must be distinct and detached from other containers".into(),
            );
        }
        if self.children() != [first.clone(), second.clone()] {
            self.clear();
            self.widget.set_start_child(Some(first));
            self.widget.set_end_child(Some(second));
        }
        Ok(())
    }
}

/// Tab choice belongs to the user; stable page keys retain it across reorder.
pub struct Tabs {
    pub widget: gtk::Notebook,
    keys: std::cell::RefCell<Vec<String>>,
    retained: std::cell::RefCell<Option<String>>,
}
impl Tabs {
    pub fn new(title: &str) -> Result<Self, String> {
        label(title)?;
        let widget = gtk::Notebook::builder().scrollable(true).build();
        widget.update_property(&[gtk::accessible::Property::Label(title)]);
        Ok(Self {
            widget,
            keys: Default::default(),
            retained: Default::default(),
        })
    }
    pub fn children(&self) -> Vec<gtk::Widget> {
        (0..self.widget.n_pages())
            .filter_map(|i| self.widget.nth_page(Some(i)))
            .collect()
    }
    pub fn selected_key(&self) -> Option<String> {
        self.widget
            .current_page()
            .and_then(|i| self.keys.borrow().get(i as usize).cloned())
    }
    pub fn select_key(&self, key: &str) -> bool {
        let index = self.keys.borrow().iter().position(|k| k == key);
        if let Some(index) = index {
            self.widget.set_current_page(Some(index as u32));
            true
        } else {
            false
        }
    }
    pub fn clear(&self) {
        if let Some(key) = self.selected_key() {
            *self.retained.borrow_mut() = Some(key);
        }
        while self.widget.n_pages() > 0 {
            self.widget.remove_page(Some(0));
        }
        self.keys.borrow_mut().clear();
    }
    pub fn set_pages(&self, pages: &[(String, String, gtk::Widget)]) -> Result<(), String> {
        let mut keys = std::collections::BTreeSet::new();
        let mut widgets = Vec::new();
        let existing = self.children();
        if pages.is_empty()
            || pages.len() > 32
            || pages.iter().any(|(key, title, widget)| {
                key.is_empty()
                    || key.len() > 128
                    || !key
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"-_:".contains(&b))
                    || !keys.insert(key)
                    || label(title).is_err()
                    || widgets.contains(widget)
                    || (widget.parent().is_some() && !existing.contains(widget))
                    || {
                        widgets.push(widget.clone());
                        false
                    }
            })
        {
            return Err("Tabs require 1–32 uniquely keyed, labeled, detached pages".into());
        }
        if existing == widgets
            && *self.keys.borrow() == pages.iter().map(|p| p.0.clone()).collect::<Vec<_>>()
        {
            for (_, title, widget) in pages {
                self.widget.set_tab_label_text(widget, title);
            }
            return Ok(());
        }
        self.clear();
        for (key, title, widget) in pages {
            self.widget
                .append_page(widget, Some(&gtk::Label::new(Some(title))));
            self.keys.borrow_mut().push(key.clone());
        }
        let selected = self.retained.borrow_mut().take();
        if !selected.is_some_and(|key| self.select_key(&key)) {
            self.widget.set_current_page(Some(0));
        }
        Ok(())
    }
}
