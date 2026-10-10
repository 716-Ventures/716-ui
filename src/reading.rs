//! Readable rich text and bounded charts with exact accessible data alternatives.
use crate::{ChartPoint, RichRun};
use gtk::prelude::*;
pub struct RichText {
    pub widget: gtk::Label,
    runs: std::cell::RefCell<Vec<RichRun>>,
}
impl RichText {
    pub fn new(runs: &[RichRun]) -> Result<Self, String> {
        let view = Self {
            widget: crate::text("", false),
            runs: Default::default(),
        };
        view.reconcile(runs)?;
        Ok(view)
    }
    pub fn reconcile(&self, runs: &[RichRun]) -> Result<bool, String> {
        if !crate::content::valid_runs(runs) {
            return Err("Invalid bounded rich text".into());
        }
        if self.widget.has_focus() || self.widget.selection_bounds().is_some() {
            return Ok(false);
        }
        if *self.runs.borrow() == runs {
            return Ok(true);
        }
        let mut markup = String::new();
        for run in runs {
            let escaped = gtk::glib::markup_escape_text(&run.text);
            let tag = match run.style.as_str() {
                "strong" => "b",
                "emphasis" => "i",
                "code" => "tt",
                _ => "",
            };
            if tag.is_empty() {
                markup.push_str(&escaped);
            } else {
                markup.push_str(&format!("<{tag}>{escaped}</{tag}>"));
            }
        }
        self.widget.set_markup(&markup);
        *self.runs.borrow_mut() = runs.to_vec();
        Ok(true)
    }
}
pub struct BoundedChart {
    pub widget: gtk::Box,
    pub plot: gtk::DrawingArea,
    pub data: crate::DataTable,
    pub heading: gtk::Label,
    points: std::rc::Rc<std::cell::RefCell<Vec<ChartPoint>>>,
}
impl BoundedChart {
    pub fn new(title: &str, points: &[ChartPoint]) -> Result<Self, String> {
        let widget = crate::column(8);
        let heading = crate::text(title, true);
        widget.append(&heading);
        let plot = gtk::DrawingArea::builder()
            .content_width(320)
            .content_height(200)
            .hexpand(true)
            .build();
        plot.update_property(&[gtk::accessible::Property::Label(title)]);
        let values = std::rc::Rc::new(std::cell::RefCell::new(Vec::<ChartPoint>::new()));
        let observed = values.clone();
        plot.set_draw_func(move |area, context, width, height| {
            let values = observed.borrow();
            if values.is_empty() {
                return;
            }
            let Some((min, max)) = crate::content::chart_domain(&values) else { return; };
            let range = max - min;
            let height = f64::from(height.max(1));
            let width = f64::from(width.max(1));
            let baseline = height * (max / range);
            let color = area.color();
            context.set_source_rgba(
                f64::from(color.red()),
                f64::from(color.green()),
                f64::from(color.blue()),
                0.85,
            );
            let slot = width / values.len() as f64;
            for (i, point) in values.iter().enumerate() {
                let top = height * ((max - point.value) / range);
                context.rectangle(
                    i as f64 * slot + slot * 0.1,
                    top.min(baseline),
                    slot * 0.8,
                    (baseline - top).abs(),
                );
            }
            let _ = context.fill();
            context.move_to(0.0, baseline);
            context.line_to(width, baseline);
            let _ = context.stroke();
        });
        widget.append(&plot);
        let data = crate::DataTable::new(title, &["Label".into(), "Value".into()])?;
        widget.append(&data.widget);
        let mut chart = Self {
            widget,
            plot,
            data,
            heading,
            points: values,
        };
        chart.update(title, points)?;
        Ok(chart)
    }
    pub fn update(&mut self, title: &str, points: &[ChartPoint]) -> Result<(), String> {
        if title.trim().is_empty()
            || title.len() > 256
            || title.contains('\0')
            || !crate::content::valid_points(points)
        {
            return Err("Invalid bounded chart".into());
        }
        let rows = points
            .iter()
            .map(|point| crate::TableRow {
                id: point.id.clone(),
                cells: vec![point.label.clone(), point.value.to_string()],
            })
            .collect::<Vec<_>>();
        self.data.update(&rows)?;
        self.heading.set_text(title);
        self.plot
            .update_property(&[gtk::accessible::Property::Label(title)]);
        self.data
            .view
            .update_property(&[gtk::accessible::Property::Label(title)]);
        *self.points.borrow_mut() = points.to_vec();
        self.plot.queue_draw();
        Ok(())
    }
}
