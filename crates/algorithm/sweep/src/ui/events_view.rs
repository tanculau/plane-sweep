use common::{segment::Segments, ui::MyWidget, ui::WidgetName};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::{Event, Step};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventsView;

impl EventsView {
    pub fn table_view(ui: &mut eframe::egui::Ui, step: &Step, segments: &Segments) {
        let mut events = step.event_queue.queue.iter();
        let total_rows = step.event_queue.queue.len();
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Y");
                });
                header.col(|ui| {
                    ui.label("X");
                });
                header.col(|ui| {
                    ui.label("Segments");
                });
                header.col(|_| {});
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let event = events.next().unwrap();
                    row.col(|ui| {
                        ui.label(event.y.to_string());
                    });
                    row.col(|ui| {
                        ui.label(event.x.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format_segment(event, segments));
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for EventsView {
    const NAME: &'static str = "Events";
}

pub struct EventsViewState<'a, 'b> {
    pub step: &'a Step,
    pub segments: &'b Segments,
}

impl<'a, 'b> MyWidget<EventsViewState<'a, 'b>> for EventsView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<EventsViewState<'a, 'b>>) {
        let EventsViewState { step, segments } = state.into();
        if let Some(event) = &step.event {
            ui.heading("Current Event:");
            ui.label(format!("Coordinate: ({:.2} , {:.2})", event.x, event.y));
            ui.label(format!("Segments: {}", format_segment(event, segments)));
            ui.separator();
        }
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view(ui, step, segments);
                    });
                });
            });
    }
}

fn format_segment(event: &Event, segments: &Segments) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    let mut s = event.segments.iter();

    if let Some(s) = s.next() {
        let _ = write!(&mut buf, "s{}", segments[*s].id);
    }
    for s in s {
        let _ = write!(&mut buf, ", s{}", segments[*s].id);
    }

    buf
}
