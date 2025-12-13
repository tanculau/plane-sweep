use common::{
    math::B,
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::beachline::BeachLine;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeachView;

impl BeachView {
    #[allow(clippy::missing_panics_doc)]
    pub fn table_view<T: B>(ui: &mut eframe::egui::Ui, beach: &BeachLine<T>) {
        let mut sites = vec![];
        beach.sites(&mut sites);
        let total_rows = sites.len();
        let mut sites = sites.into_iter();
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
                    ui.label("Arcs");
                });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let (p, q) = sites.next().unwrap();
                    row.col(|ui| {
                        ui.label(format!("Site {p} - {q:?}"));
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for BeachView {
    const NAME: &'static str = "Beach";
}

pub struct BeachViewState<'a, T: B> {
    pub current: &'a BeachLine<T>,
}

impl<'a, T: B> MyWidget<BeachViewState<'a, T>> for BeachView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<BeachViewState<'a, T>>) {
        let BeachViewState { current } = state.into();
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view(ui, current);
                    });
                });
            });
    }
}
