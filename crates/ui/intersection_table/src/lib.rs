use common::{
    AlgoStepIdx, MyWidget, WidgetName,
    intersection::{Intersection, Intersections},
    segment::Segments,
};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use tracing::info;

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IntersectionTable {
    reversed: bool,
}

impl WidgetName for IntersectionTable {
    const NAME: &'static str = "Intersection Table";
}

impl<'segments, 'intersections> MyWidget<IntersectionTableState<'segments, 'intersections>>
    for IntersectionTable
{
    #[allow(clippy::too_many_lines)]
    fn ui(
        &mut self,
        ui: &mut eframe::egui::Ui,
        state: impl Into<IntersectionTableState<'segments, 'intersections>>,
    ) {
        let state = state.into();
        let step = state.step;
        let segments = state.segments;
        let intersections = state
            .intersections
            .iter()
            .filter(|i: &&Intersection| AlgoStepIdx::from(i.step()) <= step)
            .collect::<Vec<_>>();
        let len = intersections.len();
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Index");
                        },
                        |ui| {
                            if ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked() {
                                info!("Reversing intersection table");
                                self.reversed = !self.reversed;
                            }
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("Type");
                });
                header.col(|ui| {
                    ui.strong("X");
                });
                header.col(|ui| {
                    ui.strong("Y");
                });
                header.col(|ui| {
                    ui.strong("X");
                });
                header.col(|ui| {
                    ui.strong("Y");
                });
                header.col(|ui| {
                    ui.strong("Segments");
                });
                header.col(|ui| {
                    ui.strong("Step");
                });
                header.col(|_| {});
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, len, |mut row| {
                    let row_index = if self.reversed {
                        intersections.len() - 1 - row.index()
                    } else {
                        row.index()
                    };

                    let intersection = &intersections[row_index];
                    let point1 = intersection.point1();
                    let point2 = intersection.point2();

                    // Index
                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });

                    // Typ
                    row.col(|ui| {
                        ui.label(intersection.typ().to_string());
                    });

                    // X
                    row.col(|ui| {
                        ui.label(format!("{:.2}", point1.x));
                    });
                    // Y
                    row.col(|ui| {
                        ui.label(format!("{:.2}", point1.y));
                    });
                    // X
                    row.col(|ui| {
                        if let Some(point2) = point2 {
                            ui.label(format!("{:.2}", point2.x));
                        } else {
                            ui.label("❌");
                        }
                    });
                    // Y
                    row.col(|ui| {
                        if let Some(point2) = point2.as_ref() {
                            ui.label(format!("{:.2}", point2.y));
                        } else {
                            ui.label("❌");
                        }
                    });
                    // Segments
                    row.col(|ui| {
                        let text = intersection
                            .segments()
                            .iter()
                            .map(|v| segments[*v].id)
                            .fold(String::new(), |mut i, v| {
                                use std::fmt::Write;
                                if i.is_empty() {
                                    write!(i, "s{v}").unwrap();
                                } else {
                                    write!(i, ", s{v}").unwrap();
                                }
                                i
                            });
                        ui.label(text);
                    });
                    // Step
                    row.col(|ui| {
                        ui.label(intersection.step().to_string());
                    });
                    row.col(|_| {});
                });
            });
    }
}

#[derive(Debug)]
pub struct IntersectionTableState<'segments, 'intersections> {
    pub segments: &'segments Segments,
    pub intersections: &'intersections Intersections,
    pub step: AlgoStepIdx,
}
