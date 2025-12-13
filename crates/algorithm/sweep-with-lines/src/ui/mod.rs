mod code_view;
mod merge_queue;
use core::fmt::Debug;

use common::{
    AlgoStepIdx, AlgoSteps,
    intersection::{LeanIntersections, lean_to_normal},
    math::A,
    segment::{Segment, Segments},
    ui::{MyWidget, WidgetName},
};
use controller::{Controller, ControllerState};
use eframe::egui::{self, Align, Layout, ScrollArea};
use intersection_table::{IntersectionTable, IntersectionTableState};
use itertools::chain;
use segment_plotter::{SegmentPlotter, SegmentPlotterState};
use segment_table::{SegmentTable, SegmentTableState};
use sweep_utils::ui::{
    events_view::{EventsView, EventsViewState},
    set_view::{SetView, SetViewState},
    status_view::{StatusView, StatusViewState},
};

use crate::{
    Step, calculate_steps,
    ui::{
        code_view::{CodeView, CodeViewState},
        merge_queue::{MergeQueueView, MergeQueueViewState},
    },
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(clippy::struct_excessive_bools)]
pub struct PlaneSweepOverlay<T: A> {
    step: AlgoStepIdx,
    segments: Segments<T>,
    intersections: LeanIntersections<T>,
    merged_intersections: LeanIntersections<T>,

    steps: AlgoSteps<Step<T>>,
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
    merge_queue: MergeQueueView,
    is_merge_queue_open: bool,
}

impl<T: A> WidgetName for PlaneSweepOverlay<T> {
    const NAME: &'static str = "Plane Sweep";
    const NAME_LONG: &'static str = "Plane Sweep Algorithm";
}

impl<T: A> PlaneSweepOverlay<T> {
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
        ui.toggle_value(&mut self.is_merge_queue_open, self.merge_queue.name());
    }
}

impl<T: A> MyWidget<()> for PlaneSweepOverlay<T> {
    #[allow(clippy::too_many_lines)]
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
            SegmentTableState {
                should_reset: &mut should_reset,
                segments: &mut self.segments,
            },
        );
        if should_reset {
            self.step = 0.into();
            calculate_steps::<T>(
                &self.segments,
                &mut self.intersections,
                &mut self.merged_intersections,
                &mut self.steps,
            );
        }
        self.segment_plotter.show(
            ctx,
            &mut self.is_segment_plotter_open,
            SegmentPlotterState {
                segments: &self.segments,
                intersections: &{
                    lean_to_normal(chain!(&self.intersections, &self.merged_intersections))
                },
                step: self.step,
                steps: &self.steps,
            },
        );
        self.intersection_table.show(
            ctx,
            &mut self.is_intersection_table_open,
            IntersectionTableState {
                segments: &self.segments,
                intersections: &lean_to_normal(chain!(
                    &self.intersections,
                    &self.merged_intersections
                )),
                step: self.step,
            },
        );
        self.controller.show(
            ctx,
            &mut self.is_controller_open,
            ControllerState {
                steps: &mut self.steps,
                step: &mut self.step,
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
                merged_intersections: &self.merged_intersections,
            },
        );
        MyWidget::<StatusViewState<'_, _, T>>::show(
            &mut self.status_view,
            ctx,
            &mut self.is_status_view_open,
            StatusViewState {
                step: &self.steps[self.step],
                segments: &self.segments,
            },
        );
        self.merge_queue.show(
            ctx,
            &mut self.is_merge_queue_open,
            MergeQueueViewState {
                step: &self.steps[self.step].merge_queue,
                segments: &self.segments,
            },
        );
    }
}

impl<T: A> Default for PlaneSweepOverlay<T> {
    fn default() -> Self {
        let mut out = Self {
            step: 0.into(),
            segments: [
                Segment::new((2_i8, 2_i8), (-2_i8, -2_i8)),
                Segment::new((-2_i8, 2_i8), (2_i8, -2_i8)),
                Segment::new((-1_i8, 2_i8), (-1_i8, -2_i8)),
            ]
            .into_iter()
            .collect(),
            intersections: LeanIntersections::default(),
            merged_intersections: LeanIntersections::default(),
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
            is_merge_queue_open: true,
            merge_queue: MergeQueueView,
        };
        calculate_steps::<T>(
            &out.segments,
            &mut out.intersections,
            &mut out.merged_intersections,
            &mut out.steps,
        );
        out
    }
}
