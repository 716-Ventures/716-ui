//! Read-only keyed lists and key/value groups with native row virtualization.
use crate::collections::{valid_rows, TableRow};
use gtk::{glib, prelude::*};
use std::{cell::Cell, collections::BTreeMap, rc::Rc};
pub struct DataList {
    pub widget: gtk::ScrolledWindow,
    pub view: gtk::ListView,
    store: gtk::gio::ListStore,
    selection: gtk::SingleSelection,
    objects: BTreeMap<String, glib::BoxedAnyObject>,
    rows: Vec<TableRow>,
    columns: usize,
    live_rows: Rc<Cell<usize>>,
}
impl DataList {
    pub fn new(label: &str, key_value: bool) -> Result<Self, String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded accessible collection label is required".into());
        }
        let columns = if key_value { 2 } else { 1 };
        let store = gtk::gio::ListStore::new::<glib::BoxedAnyObject>();
        let selection = gtk::SingleSelection::new(Some(store.clone()));
        selection.set_autoselect(false);
        selection.set_can_unselect(true);
        let factory = gtk::SignalListItemFactory::new();
        let live_rows = Rc::new(Cell::new(0usize));
        let count = live_rows.clone();
        factory.connect_setup(move |_, object| {
            let item = object.downcast_ref::<gtk::ListItem>().unwrap();
            let row = crate::row(16);
            row.set_margin_start(16);
            row.set_margin_end(16);
            row.set_margin_top(12);
            row.set_margin_bottom(12);
            for index in 0..columns {
                let label = crate::text("", false);
                label.set_selectable(false);
                if key_value && index == 0 {
                    label.add_css_class("heading");
                    label.set_width_chars(16);
                } else {
                    label.set_hexpand(true);
                }
                row.append(&label);
            }
            item.set_child(Some(&row));
            count.set(count.get() + 1);
        });
        factory.connect_bind(move |_, object| {
            let item = object.downcast_ref::<gtk::ListItem>().unwrap();
            let object = item
                .item()
                .unwrap()
                .downcast::<glib::BoxedAnyObject>()
                .unwrap();
            let data = object.borrow::<TableRow>();
            let mut child = item.child().unwrap().first_child();
            for cell in &data.cells {
                let label = child.take().unwrap().downcast::<gtk::Label>().unwrap();
                label.set_text(cell);
                child = label.next_sibling();
            }
        });
        factory.connect_unbind(|_, object| {
            let item = object.downcast_ref::<gtk::ListItem>().unwrap();
            let mut child = item.child().unwrap().first_child();
            while let Some(widget) = child {
                child = widget.next_sibling();
                widget.downcast::<gtk::Label>().unwrap().set_text("");
            }
        });
        let count = live_rows.clone();
        factory.connect_teardown(move |_, _| count.set(count.get().saturating_sub(1)));
        let view = gtk::ListView::new(Some(selection.clone()), Some(factory));
        view.update_property(&[gtk::accessible::Property::Label(label)]);
        view.set_show_separators(true);
        let widget = gtk::ScrolledWindow::builder()
            .min_content_height(160)
            .max_content_height(480)
            .propagate_natural_height(true)
            .child(&view)
            .build();
        Ok(Self {
            widget,
            view,
            store,
            selection,
            objects: BTreeMap::new(),
            rows: Vec::new(),
            columns,
            live_rows,
        })
    }
    pub fn selected_key(&self) -> Option<String> {
        self.selection
            .selected_item()
            .and_then(|o| o.downcast::<glib::BoxedAnyObject>().ok())
            .map(|o| o.borrow::<TableRow>().id.clone())
    }
    pub fn select_key(&self, key: &str) -> bool {
        if let Some(index) = self.rows.iter().position(|r| r.id == key) {
            self.selection.set_selected(index as u32);
            true
        } else {
            false
        }
    }
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
    pub fn live_row_count(&self) -> usize {
        self.live_rows.get()
    }
    pub fn update(&mut self, rows: &[TableRow]) -> Result<(), String> {
        if !valid_rows(rows, self.columns) {
            return Err("Invalid keyed list rows".into());
        }
        if self.rows == rows {
            return Ok(());
        }
        let selected = self.selected_key();
        let mut objects = Vec::new();
        let mut next = BTreeMap::new();
        for row in rows {
            let object = self
                .objects
                .get(&row.id)
                .cloned()
                .unwrap_or_else(|| glib::BoxedAnyObject::new(row.clone()));
            *object.borrow_mut::<TableRow>() = row.clone();
            objects.push(object.clone());
            next.insert(row.id.clone(), object);
        }
        self.store.splice(0, self.store.n_items(), &objects);
        self.objects = next;
        self.rows = rows.to_vec();
        if let Some(key) = selected {
            if !self.select_key(&key) {
                self.selection.set_selected(gtk::INVALID_LIST_POSITION);
            }
        }
        Ok(())
    }
}
