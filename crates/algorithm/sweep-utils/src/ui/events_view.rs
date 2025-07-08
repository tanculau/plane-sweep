use common::{
    math::cartesian::CartesianCoord,
    segment::{SegmentIdx, Segments},
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::event::EventQueue;

pub trait EventReport {
    fn event_queue(&self) -> &EventQueue;
    fn p(&self) -> Option<&CartesianCoord>;
    fn u_p(&self) -> &[SegmentIdx];
}

impl<T: EventReport> EventReport for &T {
    fn event_queue(&self) -> &EventQueue {
        (*self).event_queue()
    }

    fn p(&self) -> Option<&CartesianCoord> {
        (*self).p()
    }

    fn u_p(&self) -> &[SegmentIdx] {
        (*self).u_p()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventsView;

impl EventsView {
    #[allow(clippy::missing_panics_doc)]
    pub fn table_view(ui: &mut eframe::egui::Ui, report: impl EventReport, segments: &Segments) {
        let mut events = report.event_queue().queue.iter();
        let total_rows = report.event_queue().queue.len();
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
                    let (p, segs) = events.next().unwrap();
                    row.col(|ui| {
                        ui.label(p.y.to_string());
                    });
                    row.col(|ui| {
                        ui.label(p.x.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format_segment(segs.iter().copied(), segments));
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for EventsView {
    const NAME: &'static str = "Events";
}

pub struct EventsViewState<'a, 'b, T: EventReport> {
    pub step: &'a T,
    pub segments: &'b Segments,
}

impl<'a, 'b, T: EventReport> MyWidget<EventsViewState<'a, 'b, T>> for EventsView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<EventsViewState<'a, 'b, T>>) {
        let EventsViewState { step, segments } = state.into();
        if let (Some(p), segs) = (step.p(), &step.u_p()) {
            ui.heading("Current Event:");
            ui.label(format!("Coordinate: ({:.2} , {:.2})", p.x, p.y));
            ui.label(format!(
                "Segments: {}",
                format_segment(segs.iter().copied(), segments)
            ));
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

fn format_segment(mut iter: impl Iterator<Item = SegmentIdx>, segments: &Segments) -> String {
    use std::fmt::Write;
    let mut buf = String::new();

    if let Some(s) = iter.next() {
        let _ = write!(&mut buf, "s{}", segments[s].id);
    }
    for s in iter {
        let _ = write!(&mut buf, ", s{}", segments[s].id);
    }

    buf
}
