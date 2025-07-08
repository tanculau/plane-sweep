use crate::status::intersection;
use common::{
    math::cartesian::CartesianCoord,
    segment::{SegmentIdx, Segments},
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatusView;

pub trait StatusReport {
    fn status_queue(&self) -> &[SegmentIdx];
    fn p(&self) -> Option<&CartesianCoord>;
}

impl WidgetName for StatusView {
    const NAME: &'static str = "Status Queue";
}

pub struct StatusViewState<'a, T: StatusReport> {
    pub step: &'a T,
    pub segments: &'a Segments,
}

impl<'a, T: StatusReport> MyWidget<StatusViewState<'a, T>> for StatusView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<StatusViewState<'a, T>>) {
        let StatusViewState {
            step: report,
            segments,
        } = state.into();
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        let total_rows = report.status_queue().len();
                        let mut iter = report.status_queue().iter();
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
                                    ui.label("X");
                                });
                                header.col(|ui| {
                                    ui.label("Slope");
                                });
                                header.col(|ui| {
                                    ui.label("Segment");
                                });
                                header.col(|_| {});
                            })
                            .body(|body| {
                                const ROW_HEIGHT: f32 = 18.0;

                                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                                    let seg_idx = iter.next().unwrap();
                                    let seg = segments[*seg_idx].clone();
                                    row.col(|ui| {
                                        if let Some(event) = &report.p() {
                                            let x_intersect = intersection(&seg, event);
                                            ui.label(format!("{x_intersect:.2}"));
                                        } else {
                                            ui.label("");
                                        }
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", seg.slope()));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{}", seg.id));
                                    });
                                    row.col(|_| {});
                                });
                            });
                    });
                });
            });
    }
}
