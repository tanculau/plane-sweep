use common::{
    AlgoStepIdx, AlgoSteps,
    math::B,
    site::Sites,
    ui::{MyWidget, WidgetName},
};
use controller::{Controller, ControllerState};
use eframe::egui::{self, Align, Layout, ScrollArea};
use site_table::{SiteTable, SiteTableState};

use crate::{
    BoundingBox, Site, Voronoi,
    ui::{
        beach::{BeachView, BeachViewState},
        code_view::{CodeView, CodeViewState},
        dcel::{DcelView, DcelViewState},
        events::{EventsView, EventsViewState},
        plotter::{VoronoiPlotter, VoronoiPlotterState},
        steps::Step,
    },
};

mod beach;
pub mod code_view;
mod dcel;
mod events;
mod plotter;
pub mod steps;

#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct Voron<T: B + From<f64>> {
    step: AlgoStepIdx,
    sites: Sites,
    imp_sites: Vec<Site<T>>,
    bound: u32,
    steps: AlgoSteps<Step<T>>,
    site_table: SiteTable,
    is_site_table: bool,
    controller: Controller,
    is_controller_open: bool,
    plotter: VoronoiPlotter,
    is_plotter_open: bool,
    event_view: EventsView,
    is_event_view_open: bool,
    beach_view: BeachView,
    is_beach_view_open: bool,
    dcel_view: DcelView,
    is_dcel_open: bool,
    code_view: CodeView,
    is_code_view_open: bool,
}

impl<T: B + From<f64>> Default for Voron<T> {
    fn default() -> Self {
        let sites = Sites::from_iter([
            common::site::Site::new(1, 0),
            common::site::Site::new(4, -5),
            common::site::Site::new(-6, 0),
        ]);
        let bound = 20;

        let implementation = Voronoi::build(&sites, bound);
        let imp_sites = implementation.sites;
        let steps = implementation.steps;
        Self {
            step: AlgoStepIdx::from(0),
            sites,
            steps,
            imp_sites,
            bound,
            site_table: SiteTable::default(),
            is_site_table: true,
            controller: Controller::default(),
            is_controller_open: true,
            plotter: VoronoiPlotter::default(),
            is_plotter_open: true,
            event_view: EventsView,
            is_event_view_open: false,
            beach_view: BeachView,
            is_beach_view_open: false,
            dcel_view: DcelView,
            is_dcel_open: false,
            code_view: CodeView,
            is_code_view_open: false,
        }
    }
}

impl<T: B + From<f64>> Voron<T> {
    fn side_panel_groups(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                self.checkboxes(ui);
            });
        });
    }

    fn checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.toggle_value(&mut self.is_site_table, self.site_table.name());
        ui.toggle_value(&mut self.is_controller_open, self.controller.name());
        ui.toggle_value(&mut self.is_plotter_open, self.plotter.name());
        ui.toggle_value(&mut self.is_event_view_open, self.event_view.name());
        ui.toggle_value(&mut self.is_beach_view_open, self.beach_view.name());
        ui.toggle_value(&mut self.is_dcel_open, self.dcel_view.name());
        ui.toggle_value(&mut self.is_code_view_open, self.code_view.name());
    }
}

impl<T: B + From<f64>> MyWidget<()> for Voron<T> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _: impl Into<()>) {
        let ctx = ui.ctx();
        egui::SidePanel::right("Voronoi Panel")
            .resizable(false)
            .default_width(160.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Voronoi");
                });

                ui.separator();

                self.side_panel_groups(ui);
            });
        let mut should_reset = false;
        self.site_table.show(
            ctx,
            &mut self.is_site_table,
            SiteTableState {
                width: &mut self.bound,
                should_reset: &mut should_reset,
                sites: &mut self.sites,
            },
        );
        if should_reset {
            self.step = 0.into();

            let implementation = Voronoi::build(&self.sites, self.bound);
            self.steps = implementation.steps;
            self.imp_sites = implementation.sites;
        }
        self.controller.show(
            ctx,
            &mut self.is_controller_open,
            ControllerState {
                steps: &mut self.steps,
                step: &mut self.step,
            },
        );
        self.plotter.show(
            ctx,
            &mut self.is_plotter_open,
            VoronoiPlotterState {
                sites: &self.imp_sites,
                dcel: &self.steps[self.step].dcel,
                step: self.step,
                steps: &self.steps,
                bound: BoundingBox::from_width(self.bound),
                stopped: self.step == (self.steps.len() - 1).into(),
            },
        );
        self.event_view.show(
            ctx,
            &mut self.is_event_view_open,
            EventsViewState {
                current: self.steps[self.step].event,
                step: &self.steps[self.step].event_queue,
            },
        );
        self.beach_view.show(
            ctx,
            &mut self.is_beach_view_open,
            BeachViewState {
                current: &self.steps[self.step].beachline,
            },
        );
        self.dcel_view.show(
            ctx,
            &mut self.is_dcel_open,
            DcelViewState {
                current: &self.steps[self.step].dcel,
            },
        );
        self.code_view.show(
            ctx,
            &mut self.is_code_view_open,
            CodeViewState {
                step: self.step,
                steps: &self.steps,
            },
        );
    }
}

impl<T: B + From<f64>> WidgetName for Voron<T> {
    const NAME: &'static str = "Voronoi";
}
