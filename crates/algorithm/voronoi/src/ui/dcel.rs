use common::{
    math::B,
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::dcel::Dcel;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DcelView;

impl DcelView {
    #[allow(clippy::missing_panics_doc)]
    pub fn table_view_vert<T: B>(ui: &mut eframe::egui::Ui, dcel: &Dcel<T>) {
        let mut vertices = dcel.vertices.clone().into_iter().enumerate();

        let total_rows = vertices.len();
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Vertices:");
                });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let (idx, vertex) = vertices.next().unwrap();
                    row.col(|ui| {
                        ui.label(format!(
                            "Vertex {idx}: ({:.2},{:.2}), incident_edge: {}",
                            vertex.x, vertex.y, vertex.incident_edge
                        ));
                    });
                    row.col(|_| {});
                });
            });
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn table_view_half<T: B>(ui: &mut eframe::egui::Ui, dcel: &Dcel<T>) {
        let mut vertices = dcel.half_edges.clone().into_iter().enumerate();

        let total_rows = vertices.len();
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Half Edge:");
                });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let (idx, half) = vertices.next().unwrap();
                    row.col(|ui| {
                        ui.label(format!(
                            "Half Edge {idx}: next: {:?}, origin: {:?}, origin_site: {}, twin : {}",
                            half.next, half.origin, half.origin_site, half.twin
                        ));
                    });
                    row.col(|_| {});
                });
            });
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn table_view_face<T: B>(ui: &mut eframe::egui::Ui, dcel: &Dcel<T>) {
        let mut vertices = dcel.faces.clone().into_iter().enumerate();

        let total_rows = vertices.len();
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Half Edge:");
                });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let (idx, half) = vertices.next().unwrap();
                    row.col(|ui| {
                        ui.label(format!("Face {idx}: half_edge: {:?}", half));
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for DcelView {
    const NAME: &'static str = "Dcel";
}

pub struct DcelViewState<'a, T: B> {
    pub current: &'a Dcel<T>,
}

impl<'a, T: B> MyWidget<DcelViewState<'a, T>> for DcelView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<DcelViewState<'a, T>>) {
        let DcelViewState { current } = state.into();
        StripBuilder::new(ui)
            .sizes(Size::remainder(), 3)
            .vertical(|mut strip| {
                // Vertices
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view_vert(ui, current);
                    });
                });
                // Half Edges
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view_half(ui, current);
                    });
                });
                // Sites
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view_face(ui, current);
                    });
                });
            });
    }
}
