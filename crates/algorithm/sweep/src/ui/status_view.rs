use common::{
    MyWidget, WidgetName,
    segment::{SegmentIdx, Segments},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::Step;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatusView;

impl WidgetName for StatusView {
    const NAME: &'static str = "Status Queue";
}

pub struct StatusViewState<'a> {
    pub step: &'a Step,
    pub segments: &'a Segments,
}

impl<'a> MyWidget<StatusViewState<'a>> for StatusView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<StatusViewState<'a>>) {
        let StatusViewState { step, segments } = state.into();
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        let total_rows = step.status_queue.inner.len();
                        let mut iter = step.status_queue.iter();
                        let available_height = ui.available_height();
                        let table = TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .cell_layout(Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::auto())
                            .column(Column::remainder())
                            .min_scrolled_height(0.0)
                            .max_scroll_height(available_height);
                        table
                            .header(20.0, |mut header| {
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
                                    let event = iter.next().unwrap();
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", event.x_intersect.0));
                                    });
                                    row.col(|ui| {
                                        ui.label(format_segment(event.segments.iter(), segments));
                                    });
                                    row.col(|_| {});
                                });
                            });
                    });
                });
            });
    }
}

fn format_segment<'a>(a: impl Iterator<Item = &'a SegmentIdx>, segments: &Segments) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    let mut s = a;

    if let Some(s) = s.next() {
        let _ = write!(&mut buf, "s{}", segments[*s].id);
    }
    for s in s {
        let _ = write!(&mut buf, ", s{}", segments[*s].id);
    }

    buf
}
