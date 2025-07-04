use core::time::Duration;

use common::{
    ui::MyWidget, ui::WidgetName,
    segment::{Segment, SegmentIdx, Segments},
};
use eframe::egui;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use egui_notify::Toasts;
use tracing::{info, instrument, warn};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SegmentTable {
    to_delete: Vec<SegmentIdx>,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<SegmentIdx>,
    reversed: bool,
    new_p1_y: f64,
    new_p1_x: f64,
    new_p2_y: f64,
    new_p2_x: f64,
    #[cfg_attr(feature = "serde", serde(skip))]
    toasts: Toasts,
}

impl Clone for SegmentTable {
    fn clone(&self) -> Self {
        Self {
            to_delete: self.to_delete.clone(),
            scroll_to_row_slider: self.scroll_to_row_slider,
            scroll_to_row: self.scroll_to_row,
            reversed: self.reversed,
            new_p1_y: self.new_p1_y,
            new_p1_x: self.new_p1_x,
            new_p2_y: self.new_p2_y,
            new_p2_x: self.new_p2_x,
            toasts: Toasts::new(),
        }
    }
}

#[expect(clippy::missing_fields_in_debug, reason = "That is the whole reason")]
impl core::fmt::Debug for SegmentTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegmentTable")
            .field("to_delete", &self.to_delete)
            .field("scroll_to_row_slider", &self.scroll_to_row_slider)
            .field("scroll_to_row", &self.scroll_to_row)
            .field("reversed", &self.reversed)
            .field("new_p1_y", &self.new_p1_y)
            .field("new_p1_x", &self.new_p1_x)
            .field("new_p2_y", &self.new_p2_y)
            .field("new_p2_x", &self.new_p2_x)
            .finish()
    }
}

impl SegmentTable {
    #[allow(clippy::too_many_lines)]
    #[instrument(
        name = "segment_table",
        skip(self, ui, disable_all, should_reset, segments)
    )]
    fn table_ui(
        &mut self,
        ui: &mut egui::Ui,
        disable_all: bool,
        should_reset: &mut bool,
        segments: &mut Segments,
    ) {
        if disable_all {
            for segment in segments.iter_mut() {
                segment.shown = false;
                *should_reset = true;
            }
        }
        self.to_delete.sort_by(|v1, v2| v1.cmp(v2).reverse());
        for idx in self.to_delete.drain(..) {
            segments.remove(idx);
        }
        let total_rows = segments.len();
        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
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
        if let Some(row_index) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_index.into(), None);
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Index");
                });
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Id");
                        },
                        |ui| {
                            if ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked() {
                                self.reversed = !self.reversed;
                                info!("Reversing segment table order: {}", self.reversed);
                            }
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("Upper-X");
                });
                header.col(|ui| {
                    ui.strong("Upper-Y");
                });
                header.col(|ui| {
                    ui.strong("Lower-X");
                });
                header.col(|ui| {
                    ui.strong("Lower-Y");
                });
                header.col(|ui| {
                    ui.strong("Active");
                });
                header.col(|ui| {
                    ui.strong("Delete");
                });
                header.col(|_| {});
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let row_index = if self.reversed {
                        total_rows - 1 - row.index()
                    } else {
                        row.index()
                    };
                    let segment: &mut Segment = &mut segments[SegmentIdx::from(row_index)];

                    row.set_selected(segment.mark);

                    // Index
                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });

                    // Id
                    row.col(|ui| {
                        ui.label(segment.id.to_string());
                    });

                    // Upper X
                    row.col(|ui| {
                        let x = &mut segment.upper.x;

                        //if ui.add(egui::DragValue::new(x)).changed() {
                        //    *should_reset |= true;
                        //    info!("Updating upper Y of segment {} to {}", segment.id, x);
                        //    segment.update();
                        //}
                    });
                    // Upper Y
                    row.col(|ui| {
                        //let y = &mut segment.upper.y;
                        //if ui.add(egui::DragValue::new(y)).changed() {
                        //    *should_reset |= true;
                        //    info!("Updating upper X of segment {} to {}", segment.id, y);
                        //    segment.update();
                        //}
                    });
                    // Lower X
                    row.col(|ui| {
                        //let x = &mut segment.lower.x;
                        //if ui.add(egui::DragValue::new(x)).changed() {
                        //    *should_reset |= true;
                        //    info!("Updating lower Y of segment {} to {}", segment.id, x);
                        //    segment.update();
                        //}
                    });
                    // Lower Y
                    row.col(|ui| {
                        //let y = &mut segment.lower.y;
                        //if ui.add(egui::DragValue::new(y)).changed() {
                        //    *should_reset |= true;
                        //    info!("Updating lower X of segment {} to {}", segment.id, y);
                        //    segment.update();
                        //}
                    });
                    // Active
                    row.col(|ui| {
                        let checked = &mut segment.shown;
                        let response = ui.checkbox(checked, "Active").changed();
                        *should_reset |= response;
                        if response {
                            info!(
                                "Updating active state of segment {} to {}",
                                segment.id, checked
                            );
                        }
                    });
                    row.col(|ui| {
                        if ui.button("Delete").clicked() {
                            self.to_delete.push(row_index.into());
                            *should_reset |= true;
                            info!("Marked segment {} for deletion", segment.id);
                        }
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for SegmentTable {
    const NAME: &'static str = "Segment Table";
}

impl<'reset, 'segment> MyWidget<SegmentTableState<'reset, 'segment>> for SegmentTable {
    #[instrument(name = "segment_table", skip(self, ui, state))]
    fn ui(
        &mut self,
        ui: &mut eframe::egui::Ui,
        state: impl Into<SegmentTableState<'reset, 'segment>>,
    ) {
        let SegmentTableState {
            should_reset,
            segments,
        } = state.into();
        let disable_all = ui.button("Disable All Lines").clicked();
        if disable_all {
            info!("Disabling all segments");
        }

        self.toasts.show(ui.ctx());

        let slider_response = ui.add(
            egui::Slider::new(&mut self.scroll_to_row_slider, 0..=segments.len())
                .logarithmic(true)
                .text("Row to scroll to"),
        );
        if slider_response.changed() {
            self.scroll_to_row = Some(self.scroll_to_row_slider.into());
        }
        ui.separator();
        ui.vertical(|ui| {
            if ui.button("Create Segment").clicked() {
                *should_reset |= true;
                let segment = Segment::new(
                    (self.new_p1_x, self.new_p1_y),
                    (self.new_p2_x, self.new_p2_y),
                );

                if segment.upper ==  segment.lower  {
                    warn!(
                        "Tried creating a illegal with points ({}, {}) and ({}, {})",
                        self.new_p1_x, self.new_p1_y, self.new_p2_x, self.new_p2_y
                    );
                    self.toasts
                        .error("Segment needs to have two different points")
                        .duration(Some(Duration::from_secs(5)))
                        .closable(true);
                } else {
                    info!(
                        "Creating new segment {} with points ({}, {}) and ({}, {})",
                        segment.id, self.new_p1_x, self.new_p1_y, self.new_p2_x, self.new_p2_y
                    );
                    segments.push(segment);

                    self.new_p1_x = 0.into();
                    self.new_p1_y = 0.into();
                    self.new_p2_x = 0.into();
                    self.new_p2_y = 0.into();
                }
            }
            ui.horizontal(|ui| {
                ui.label("Point 1:");
                ui.label(" X:");
                ui.add(egui::Slider::new(&mut self.new_p1_x, -255.0..=255.0));
                ui.label(" Y:");
                ui.add(egui::Slider::new(&mut self.new_p1_y, -255.0..=255.0));
            });
            ui.horizontal(|ui| {
                ui.label("Point 2:");
                ui.label(" X:");
                ui.add(egui::Slider::new(&mut self.new_p2_x, -255.0..=255.0));
                ui.label(" Y:");
                ui.add(egui::Slider::new(&mut self.new_p2_y, -255.0..=255.0));
            });
        });
        ui.separator();
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui, disable_all, should_reset, segments);
                    });
                });
            });
    }
}

#[derive(Debug)]
pub struct SegmentTableState<'reset, 'segment> {
    should_reset: &'reset mut bool,
    segments: &'segment mut Segments,
}

impl<'reset, 'segment> From<(&'reset mut bool, &'segment mut Segments)>
    for SegmentTableState<'reset, 'segment>
{
    fn from((should_reset, segments): (&'reset mut bool, &'segment mut Segments)) -> Self {
        Self::new(should_reset, segments)
    }
}

impl<'reset, 'segment> SegmentTableState<'reset, 'segment> {
    #[must_use]
    pub const fn new(should_reset: &'reset mut bool, segments: &'segment mut Segments) -> Self {
        Self {
            should_reset,
            segments,
        }
    }
}
