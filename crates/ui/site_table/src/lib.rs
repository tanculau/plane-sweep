// Based on https://github.com/emilk/egui/blob/a0bb4cfef82dd9b50f990f607b7c7c4f28eb8589/crates/egui_demo_lib/src/demo/table_demo.rs

use core::time::Duration;

use common::{
    site::{Site, SiteIdx, Sites},
    ui::{MyWidget, WidgetName},
};
use eframe::egui;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use egui_notify::Toasts;
use tracing::{info, instrument, warn};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SiteTable {
    to_delete: Vec<SiteIdx>,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<SiteIdx>,
    reversed: bool,
    new_p1_y: i32,
    new_p1_x: i32,
    bound: u32,
    #[cfg_attr(feature = "serde", serde(skip))]
    toasts: Toasts,
}

impl Default for SiteTable {
    fn default() -> Self {
        Self {
            to_delete: Vec::new(),
            scroll_to_row_slider: Default::default(),
            scroll_to_row: None,
            reversed: Default::default(),
            new_p1_y: Default::default(),
            new_p1_x: Default::default(),
            bound: 10,
            toasts: Toasts::default(),
        }
    }
}

impl Clone for SiteTable {
    fn clone(&self) -> Self {
        Self {
            to_delete: self.to_delete.clone(),
            scroll_to_row_slider: self.scroll_to_row_slider,
            scroll_to_row: self.scroll_to_row,
            reversed: self.reversed,
            new_p1_y: self.new_p1_y,
            new_p1_x: self.new_p1_x,
            bound: self.bound,
            toasts: Toasts::new(),
        }
    }
}

#[expect(clippy::missing_fields_in_debug, reason = "That is the whole reason")]
impl core::fmt::Debug for SiteTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Site Table")
            .field("to_delete", &self.to_delete)
            .field("scroll_to_row_slider", &self.scroll_to_row_slider)
            .field("scroll_to_row", &self.scroll_to_row)
            .field("reversed", &self.reversed)
            .field("new_p1_y", &self.new_p1_y)
            .field("new_p1_x", &self.new_p1_x)
            .finish()
    }
}

impl SiteTable {
    #[allow(clippy::too_many_lines)]
    #[instrument(name = "segment_table", skip(self, ui, should_reset, sites))]
    fn table_ui(&mut self, ui: &mut egui::Ui, should_reset: &mut bool, sites: &mut Sites) {
        self.to_delete.sort_by(|v1, v2| v1.cmp(v2).reverse());
        for idx in self.to_delete.drain(..) {
            sites.remove(idx);
        }
        let old_sites = sites.clone();
        let total_rows = old_sites.len();
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
                                info!("Reversing site table order: {}", self.reversed);
                            }
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("X");
                });
                header.col(|ui| {
                    ui.strong("Y");
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
                    let segment: Site = old_sites[SiteIdx::from(row_index)];

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
                        let x = &segment.x;
                        ui.label(format!("{x:.2}"));
                        //if ui.add(egui::DragValue::new(x)).changed() {
                        //    *should_reset |= true;
                        //    info!("Updating upper Y of segment {} to {}", segment.id, x);
                        //    segment.update();
                        //}
                    });
                    // Upper Y
                    row.col(|ui| {
                        let y = &segment.y;
                        ui.label(format!("{y:.2}"));

                        //if ui.add(egui::DragValue::new(y)).changed() {
                        //    *should_reset |= true;
                        //    info!("Updating upper X of segment {} to {}", segment.id, y);
                        //    segment.update();
                        //}
                    });

                    row.col(|ui| {
                        if ui.button("Delete").clicked() {
                            if sites.len() > row_index {
                                sites.remove(row_index.into());
                            }
                            *should_reset |= true;
                        }
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for SiteTable {
    const NAME: &'static str = "Site Table";
}

impl<'reset, 'segment, 'b> MyWidget<SiteTableState<'reset, 'segment, 'b>> for SiteTable {
    #[instrument(name = "segment_table", skip(self, ui, state))]
    fn ui(
        &mut self,
        ui: &mut eframe::egui::Ui,
        state: impl Into<SiteTableState<'reset, 'segment, 'b>>,
    ) {
        let SiteTableState {
            should_reset,
            sites,
            width,
        } = state.into();
        let disable_all = ui.button("Disable All Lines").clicked();
        if disable_all {
            info!("Disabling all segments");
        }

        self.toasts.show(ui.ctx());

        let slider_response = ui.add(
            egui::Slider::new(&mut self.scroll_to_row_slider, 0..=sites.len())
                .logarithmic(true)
                .text("Row to scroll to"),
        );
        if slider_response.changed() {
            self.scroll_to_row = Some(self.scroll_to_row_slider.into());
        }
        ui.separator();
        ui.vertical(|ui| {
            if ui.button("Create Site").clicked() {
                *should_reset |= true;
                let site = Site::new(self.new_p1_x, self.new_p1_y);
                sites.push(site);
                self.new_p1_x = 0;
                self.new_p1_y = 0;
            }
            ui.horizontal(|ui| {
                ui.label("Point 1:");
                ui.label(" X:");
                ui.add(egui::Slider::new(&mut self.new_p1_x, -255..=255));
                ui.label(" Y:");
                ui.add(egui::Slider::new(&mut self.new_p1_y, -255..=255));
            });
        });
        ui.vertical(|ui| {
            if ui.button("Set Bounding Box").clicked() {
                *should_reset |= true;

                if self.bound > 0 {
                    *width = self.bound;
                } else {
                    self.toasts
                        .error("Bounding Box must width must be at least 1")
                        .duration(Some(Duration::from_secs(5)))
                        .closable(true);
                }
            }
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(egui::Slider::new(&mut self.bound, 1..=255));
            });
        });
        ui.separator();
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui, should_reset, sites);
                    });
                });
            });
    }
}

#[derive(Debug)]
pub struct SiteTableState<'reset, 'segment, 'b> {
    pub should_reset: &'reset mut bool,
    pub sites: &'segment mut Sites,
    pub width: &'b mut u32,
}
