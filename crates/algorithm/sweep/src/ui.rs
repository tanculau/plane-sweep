mod code_view;
mod events_view;
mod set_view;
mod status_view;

use common::{
    AlgoStepIdx, AlgoSteps, MyWidget, WidgetName,
    intersection::Intersections,
    segment::{Segment, Segments},
};
use controller::{Controller, ControllerState};
use eframe::egui::{self, Align, Layout, ScrollArea};
use intersection_table::{IntersectionTable, IntersectionTableState};
use segment_plotter::{SegmentPlotter, SegmentPlotterState};
use segment_table::SegmentTable;

use crate::{
    Step, calculate_steps,
    ui::{
        code_view::{CodeView, CodeViewState},
        events_view::{EventsView, EventsViewState},
        set_view::{SetView, SetViewState},
        status_view::{StatusView, StatusViewState},
    },
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(clippy::struct_excessive_bools)]
pub struct PlaneSweep {
    step: AlgoStepIdx,
    segments: Segments,
    intersections: Intersections,
    steps: AlgoSteps<Step>,
    #[cfg_attr(feature = "serde", serde(skip))]
    controller: Controller,
    is_controller_open: bool,
    segment_plotter: SegmentPlotter,
    is_segment_plotter_open: bool,
    intersection_table: IntersectionTable,
    is_intersection_table_open: bool,
    segment_table: SegmentTable,
    is_segment_table_open: bool,
    set_view: SetView,
    is_set_view_open: bool,
    events_view: EventsView,
    is_events_view_open: bool,
    code_view: CodeView,
    is_code_view_open: bool,
    status_view: StatusView,
    is_status_view_open: bool,
}

impl WidgetName for PlaneSweep {
    const NAME: &'static str = "Plane Sweep";
    const NAME_LONG: &'static str = "Plane Sweep Algorithm";
}

impl PlaneSweep {
    fn side_panel_groups(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                self.checkboxes(ui);
            });
        });
    }

    fn checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.toggle_value(
            &mut self.is_segment_plotter_open,
            self.segment_plotter.name(),
        );
        ui.toggle_value(&mut self.is_segment_table_open, self.segment_table.name());
        ui.toggle_value(
            &mut self.is_intersection_table_open,
            self.intersection_table.name(),
        );
        ui.toggle_value(&mut self.is_controller_open, self.controller.name());
        ui.toggle_value(&mut self.is_set_view_open, self.set_view.name());
        ui.toggle_value(&mut self.is_events_view_open, self.events_view.name());
        ui.toggle_value(&mut self.is_code_view_open, self.code_view.name());
    }
}

impl MyWidget<()> for PlaneSweep {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _: impl Into<()>) {
        let ctx = ui.ctx();
        egui::SidePanel::right("Plane Sweep Panel")
            .resizable(false)
            .default_width(160.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Plane Sweep");
                });

                ui.separator();

                self.side_panel_groups(ui);
            });
        let mut should_reset = false;
        self.segment_table.show(
            ctx,
            &mut self.is_segment_table_open,
            (&mut should_reset, &mut self.segments),
        );
        if should_reset {
            self.step = 0.into();
            calculate_steps(&self.segments, &mut self.intersections, &mut self.steps);
        }
        self.segment_plotter.show(
            ctx,
            &mut self.is_segment_plotter_open,
            SegmentPlotterState {
                segments: &self.segments,
                intersections: &self.intersections,
                step: self.step,
                steps: &self.steps,
            },
        );
        self.intersection_table.show(
            ctx,
            &mut self.is_segment_plotter_open,
            IntersectionTableState {
                segments: &self.segments,
                intersections: &self.intersections,
                step: self.step,
            },
        );
        self.controller.show(
            ctx,
            &mut self.is_controller_open,
            ControllerState {
                steps: &mut self.steps,
                step: &mut self.step,
                intersections: &mut self.intersections,
            },
        );
        self.set_view.show(
            ctx,
            &mut self.is_set_view_open,
            SetViewState {
                step: &self.steps[self.step],
                segments: &self.segments,
            },
        );
        self.events_view.show(
            ctx,
            &mut self.is_events_view_open,
            EventsViewState {
                step: &self.steps[self.step],
                segments: &self.segments,
            },
        );
        self.code_view.show(
            ctx,
            &mut self.is_code_view_open,
            CodeViewState {
                step: self.step,
                steps: &self.steps,
                segments: &self.segments,
                intersections: &self.intersections,
            },
        );
        self.status_view.show(
            ctx,
            &mut self.is_status_view_open,
            StatusViewState {
                step: &self.steps[self.step],
                segments: &self.segments,
            },
        );
    }
}

impl Default for PlaneSweep {
    fn default() -> Self {
        let mut out = Self {
            step: 0.into(),
            segments: [
                Segment::new((2, 2), (-2, -2)),
                Segment::new((-2, 2), (2, -2)),
                Segment::new((-1, 2), (-1, -2)),
            ]
            .into_iter()
            .collect(),
            intersections: Intersections::default(),
            steps: AlgoSteps::default(),
            controller: Controller::default(),
            is_controller_open: true,
            segment_plotter: SegmentPlotter::default(),
            is_segment_plotter_open: true,
            intersection_table: IntersectionTable::default(),
            is_intersection_table_open: true,
            segment_table: SegmentTable::default(),
            is_segment_table_open: true,
            set_view: SetView,
            is_set_view_open: true,
            events_view: EventsView,
            is_events_view_open: true,
            code_view: CodeView,
            is_code_view_open: true,
            status_view: StatusView,
            is_status_view_open: true,
        };
        calculate_steps(&out.segments, &mut out.intersections, &mut out.steps);
        out
    }
}
