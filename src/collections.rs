//! Bounded data collections with stable row keys and native GTK virtualization.
use gtk::{glib, prelude::*};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
}
pub struct DataTable {
    pub widget: gtk::ScrolledWindow,
    pub view: gtk::ColumnView,
    store: gtk::gio::ListStore,
    selection: gtk::SingleSelection,
    objects: BTreeMap<String, glib::BoxedAnyObject>,
    rows: Vec<TableRow>,
    columns: Vec<String>,
    live_cells: std::rc::Rc<std::cell::Cell<usize>>,
    copy_menu: gtk::Popover,
}
impl DataTable {
    pub fn new(label: &str, columns: &[String]) -> Result<Self, String> {
        if label.trim().is_empty() || label.len() > 256 || label.contains('\0') {
            return Err("A bounded accessible collection label is required".into());
        }
        if !valid_columns(columns) {
            return Err("Use 1–32 bounded column labels".into());
        }
        let store = gtk::gio::ListStore::new::<glib::BoxedAnyObject>();
        let selection = gtk::SingleSelection::new(Some(store.clone()));
        selection.set_autoselect(false);
        selection.set_can_unselect(true);
        let view = gtk::ColumnView::new(Some(selection.clone()));
        view.add_css_class("seven-table");
        view.update_property(&[gtk::accessible::Property::Label(label)]);
        view.set_show_column_separators(false);
        view.set_show_row_separators(true);
        let live_cells = std::rc::Rc::new(std::cell::Cell::new(0usize));
        for (index, title) in columns.iter().enumerate() {
            let factory = gtk::SignalListItemFactory::new();
            let count = live_cells.clone();
            factory.connect_setup(move |_, object| {
                let item = object.downcast_ref::<gtk::ListItem>().unwrap();
                let label = crate::text("", false);
                label.set_selectable(false);
                label.set_wrap(false);
                label.set_margin_top(8);
                label.set_margin_bottom(8);
                label.set_margin_start(8);
                label.set_margin_end(8);
                item.set_child(Some(&label));
                count.set(count.get() + 1);
            });
            factory.connect_bind(move |_, object| {
                let item = object.downcast_ref::<gtk::ListItem>().unwrap();
                let object = item
                    .item()
                    .unwrap()
                    .downcast::<glib::BoxedAnyObject>()
                    .unwrap();
                let row = object.borrow::<TableRow>();
                let label = item.child().unwrap().downcast::<gtk::Label>().unwrap();
                label.set_text(&row.cells[index]);
            });
            factory.connect_unbind(|_, object| {
                object
                    .downcast_ref::<gtk::ListItem>()
                    .unwrap()
                    .child()
                    .unwrap()
                    .downcast::<gtk::Label>()
                    .unwrap()
                    .set_text("");
            });
            let count = live_cells.clone();
            factory.connect_teardown(move |_, _| count.set(count.get().saturating_sub(1)));
            let column = gtk::ColumnViewColumn::new(Some(title), Some(factory));
            column.set_expand(true);
            column.set_resizable(true);
            view.append_column(&column);
        }
        let widget = gtk::ScrolledWindow::builder()
            .min_content_height(160)
            .max_content_height(480)
            .propagate_natural_height(true)
            .child(&view)
            .build();
        let copy_menu = crate::table_actions::install(&view, &selection);
        Ok(Self {
            widget,
            view,
            store,
            selection,
            objects: BTreeMap::new(),
            rows: Vec::new(),
            columns: columns.to_vec(),
            live_cells,
            copy_menu,
        })
    }
    pub fn columns(&self) -> &[String] {
        &self.columns
    }
    pub fn selected_key(&self) -> Option<String> {
        self.selection
            .selected_item()
            .and_then(|object| object.downcast::<glib::BoxedAnyObject>().ok())
            .map(|object| object.borrow::<TableRow>().id.clone())
    }
    pub fn select_key(&self, key: &str) -> bool {
        if let Some(index) = self.rows.iter().position(|row| row.id == key) {
            self.selection.set_selected(index as u32);
            true
        } else {
            false
        }
    }
    pub fn live_cell_count(&self) -> usize {
        self.live_cells.get()
    }
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
    pub fn update(&mut self, rows: &[TableRow]) -> Result<(), String> {
        if !valid_rows(rows, self.columns.len()) {
            return Err("Invalid keyed table rows".into());
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
pub fn valid_columns(columns: &[String]) -> bool {
    !columns.is_empty()
        && columns.len() <= 32
        && columns
            .iter()
            .all(|s| !s.is_empty() && s.len() <= 256 && !s.contains('\0'))
}
pub fn valid_rows(rows: &[TableRow], columns: usize) -> bool {
    let mut keys = BTreeSet::new();
    rows.len() <= 10000
        && (1..=32).contains(&columns)
        && rows.iter().all(|row| {
            !row.id.is_empty()
                && row.id.len() <= 128
                && row
                    .id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_:".contains(&b))
                && keys.insert(&row.id)
                && row.cells.len() == columns
                && row
                    .cells
                    .iter()
                    .all(|cell| cell.len() <= 4096 && !cell.contains('\0'))
        })
}

impl Drop for DataTable {
    fn drop(&mut self) {
        self.copy_menu.unparent();
    }
}
