use common::{
    math::A,
    segment::{SegmentIdx, Segments},
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::SortedIntersections;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MergeQueueView;

impl MergeQueueView {
    #[allow(clippy::missing_panics_doc, clippy::ptr_arg)]
    pub fn table_view<T: A>(
        ui: &mut eframe::egui::Ui,
        queue: &Vec<([SegmentIdx; 2], SortedIntersections<T>)>,
        segments: &Segments<T>,
    ) {
        let total_rows = queue.len();
        let mut events = queue.iter();
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
                    ui.label("Event");
                });
            })
            .body(|body| {
                const ROW_HEIGHT: f32 = 18.0;
                body.rows(ROW_HEIGHT, total_rows, |mut row| {
                    let ([s1, s2], i) = events.next().unwrap();
                    row.col(|ui| {
                        ui.label(format!("s{},s{}: {i}", segments[*s1].id, segments[*s2].id));
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for MergeQueueView {
    const NAME: &'static str = "Merge Queue";
}

pub struct MergeQueueViewState<'a, 'b, T: A> {
    pub step: &'a Vec<([SegmentIdx; 2], SortedIntersections<T>)>,
    pub segments: &'b Segments<T>,
}

impl<'a, 'b, T: A> MyWidget<MergeQueueViewState<'a, 'b, T>> for MergeQueueView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<MergeQueueViewState<'a, 'b, T>>) {
        let MergeQueueViewState {
            step: queue,
            segments,
        } = state.into();
        StripBuilder::new(ui)
            .sizes(Size::remainder(), 1)
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view(ui, queue, segments);
                    });
                });
            });
    }
}
