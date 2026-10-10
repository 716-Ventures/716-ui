//! A shared plain-text document editor. Filesystem import/export belongs to the host.
use gtk::prelude::*;
pub struct DocumentEditor {
    pub widget: gtk::Box,
    pub area: crate::TextArea,
    pub find: gtk::Entry,
    pub replacement: gtk::Entry,
    pub next: gtk::Button,
    pub replace: gtk::Button,
    pub replace_all: gtk::Button,
    pub notice: gtk::Label,
}
impl DocumentEditor {
    pub fn new(label: &str, value: &str) -> Result<Self, String> {
        if label.trim().is_empty()
            || label.len() > 256
            || label.contains('\0')
            || value.len() > 65536
            || value.contains('\0')
        {
            return Err("A bounded document label and UTF-8 text are required".into());
        }
        let widget = crate::column(8);
        widget.update_property(&[gtk::accessible::Property::Label(label)]);
        let find = gtk::Entry::builder()
            .placeholder_text("Find literal text")
            .max_length(256)
            .build();
        find.update_property(&[gtk::accessible::Property::Label("Find in document")]);
        let replacement = gtk::Entry::builder()
            .placeholder_text("Replacement")
            .max_length(256)
            .build();
        replacement.update_property(&[gtk::accessible::Property::Label("Replace with")]);
        let next = crate::button("Find next", crate::ButtonVariant::Outline, false);
        let replace = crate::button("Replace selection", crate::ButtonVariant::Outline, false);
        let replace_all = crate::button("Replace all", crate::ButtonVariant::Outline, false);
        let tools = gtk::FlowBox::new();
        tools.set_selection_mode(gtk::SelectionMode::None);
        tools.set_min_children_per_line(1);
        tools.set_max_children_per_line(3);
        tools.set_row_spacing(6);
        tools.set_column_spacing(6);
        for control in [
            find.clone().upcast::<gtk::Widget>(),
            replacement.clone().upcast(),
            next.clone().upcast(),
            replace.clone().upcast(),
            replace_all.clone().upcast(),
        ] {
            tools.insert(&control, -1);
        }
        widget.append(&tools);
        let area = crate::TextArea::new(label, value);
        area.view.set_monospace(true);
        area.view.set_vexpand(true);
        let scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .min_content_height(240)
            .max_content_height(640)
            .propagate_natural_height(true)
            .child(&area.widget)
            .build();
        widget.append(&scroll);
        let notice = crate::text("Edits remain local until saved by the host", false);
        widget.append(&notice);
        crate::editor_bounds::install(&area.view.buffer(), &notice);

        crate::clipboard::install(&area.view, &notice);

        let (query, view, message) = (find.clone(), area.view.clone(), notice.clone());
        next.connect_clicked(move |_| {
            let query = query.text();
            if query.is_empty() || query.len() > 256 {
                message.set_text("Enter 1–256 bytes of literal search text");
                return;
            }
            let buffer = view.buffer();
            let cursor = buffer.iter_at_offset(buffer.cursor_position());
            let found = cursor
                .forward_search(&query, gtk::TextSearchFlags::TEXT_ONLY, None)
                .or_else(|| {
                    buffer.start_iter().forward_search(
                        &query,
                        gtk::TextSearchFlags::TEXT_ONLY,
                        None,
                    )
                });
            if let Some((mut start, end)) = found {
                buffer.select_range(&end, &start);
                view.scroll_to_iter(&mut start, 0.1, false, 0.0, 0.0);
                message.set_text("Match selected");
            } else {
                message.set_text("No literal match");
            }
        });
        let (query, insert_text, view, message) = (
            find.clone(),
            replacement.clone(),
            area.view.clone(),
            notice.clone(),
        );
        replace.connect_clicked(move |_| {
            let buffer = view.buffer();
            let query = query.text();
            let replacement = insert_text.text();
            let Some((mut start, mut end)) = buffer.selection_bounds() else {
                message.set_text("Find and select a matching occurrence first");
                return;
            };
            if query.is_empty()
                || query.len() > 256
                || replacement.len() > 256
                || buffer.text(&start, &end, true) != query
            {
                message.set_text("Selection must equal the literal search text");
                return;
            }
            let current = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
            if current.len() - query.len() + replacement.len() > 65536 {
                message.set_text("Replacement would exceed 64 KiB");
                return;
            }
            buffer.begin_user_action();
            buffer.delete(&mut start, &mut end);
            buffer.insert(&mut start, &replacement);
            buffer.end_user_action();
            message.set_text("One occurrence replaced · edit remains local");
        });
        let (query, insert_text, view, message) = (
            find.clone(),
            replacement.clone(),
            area.view.clone(),
            notice.clone(),
        );
        replace_all.connect_clicked(move |_| {
            let buffer = view.buffer();
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
            match crate::content::replace_literal(&text, &query.text(), &insert_text.text()) {
                Ok((text, count)) => {
                    if count > 0 {
                        buffer.begin_user_action();
                        let (mut start, mut end) = (buffer.start_iter(), buffer.end_iter());
                        buffer.delete(&mut start, &mut end);
                        buffer.insert(&mut start, &text);
                        buffer.end_user_action();
                    }
                    message.set_text(&format!(
                        "{count} occurrences replaced · edits remain local"
                    ));
                }
                Err(error) => message.set_text(&error),
            }
        });
        Ok(Self {
            widget,
            area,
            find,
            replacement,
            next,
            replace,
            replace_all,
            notice,
        })
    }
}
