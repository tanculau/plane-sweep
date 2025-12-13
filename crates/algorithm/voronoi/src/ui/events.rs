use common::{
    math::B,
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Layout};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::event_queue::{Event, EventQueue};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventsView;

impl EventsView {
    #[allow(clippy::missing_panics_doc)]
    pub fn table_view<T: B>(ui: &mut eframe::egui::Ui, queue: &EventQueue<T>) {
        let mut events = vec![];
        let mut queue = queue.clone();

        while let Some(event) = queue.pop() {
            events.push(event);
        }
        let total_rows = events.len();
        let mut events = events.into_iter();
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
                    let p = events.next().unwrap();
                    row.col(|ui| {
                        ui.label(p.to_string());
                    });
                    row.col(|_| {});
                });
            });
    }
}

impl WidgetName for EventsView {
    const NAME: &'static str = "Events";
}

pub struct EventsViewState<'a, T: B> {
    pub current: Option<Event<T>>,
    pub step: &'a EventQueue<T>,
}

impl<'a, T: B> MyWidget<EventsViewState<'a, T>> for EventsView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<EventsViewState<'a, T>>) {
        let EventsViewState {
            step: queue,
            current,
        } = state.into();
        if let Some(event) = current {
            ui.heading("Current Event:");
            ui.label(format!("{event}"));
            ui.separator();
        }
        StripBuilder::new(ui)
            .sizes(Size::remainder(), 1)
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        Self::table_view(ui, queue);
                    });
                });
            });
    }
}
