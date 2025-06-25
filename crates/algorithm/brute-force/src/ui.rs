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

use crate::{AlgorithmStep, calculate_steps};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(clippy::struct_excessive_bools)]
pub struct BruteForce {
    step: AlgoStepIdx,
    segments: Segments,
    intersections: Intersections,
    steps: AlgoSteps<AlgorithmStep>,
    controller: Controller,
    is_controller_open: bool,
    segment_plotter: SegmentPlotter,
    is_segment_plotter_open: bool,
    intersection_table: IntersectionTable,
    is_intersection_table_open: bool,
    segment_table: SegmentTable,
    is_segment_table_open: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    code_viewer: CodeViewer,
    is_code_viewer_open: bool,
}

impl Default for BruteForce {
    fn default() -> Self {
        let mut ret = Self {
            step: 0.into(),
            segments: [
                Segment::new((0, 0), (12, 12)),
                Segment::new((-25, 12), (112, -12)),
                Segment::new((0, 12), (112, -12)),
                Segment::new((33, 33), (96, -12)),
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
            code_viewer: CodeViewer::default(),
            is_code_viewer_open: true,
        };
        calculate_steps(&ret.segments, &mut ret.intersections, &mut ret.steps);
        ret
    }
}
impl BruteForce {
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
        ui.toggle_value(&mut self.is_code_viewer_open, self.code_viewer.name());
    }
}

impl WidgetName for BruteForce {
    const NAME: &'static str = "Brute Force";
    const NAME_LONG: &'static str = "Brute Force Algorithm";
}

impl MyWidget<()> for BruteForce {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _state: impl Into<()>) {
        let ctx = ui.ctx();
        egui::SidePanel::right("Brute Force Panel")
            .resizable(false)
            .default_width(160.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Brute Force");
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
            },
        );
        self.intersection_table.show(
            ctx,
            &mut self.is_intersection_table_open,
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
        self.code_viewer.show(
            ctx,
            &mut self.is_code_viewer_open,
            CodeViewerState {
                steps: &self.steps,
                segments: &self.segments,
                intersections: &self.intersections,
                step: self.step,
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct CodeViewer {
    buf: String,
    last_step: AlgoStepIdx,
}

impl Default for CodeViewer {
    fn default() -> Self {
        Self {
            buf: String::new(),
            last_step: 0.into(),
        }
    }
}

impl WidgetName for CodeViewer {
    const NAME: &'static str = "Visualizer";
    const NAME_LONG: &'static str = "Code Visualizer";
}

#[derive(Debug, Clone, Copy)]
pub struct CodeViewerState<'a> {
    steps: &'a AlgoSteps<AlgorithmStep>,
    segments: &'a Segments,
    intersections: &'a Intersections,
    step: AlgoStepIdx,
}

impl<'a> MyWidget<CodeViewerState<'a>> for CodeViewer {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<CodeViewerState<'a>>) {
        self.code(state.into());
        show_code(ui, &self.buf);
    }
}

impl CodeViewer {
    pub fn code(&mut self, state: CodeViewerState<'_>) {
        use std::fmt::Write;

        // Step already calculated
        if state.step == self.last_step && !self.buf.is_empty() {
            return;
        }

        self.last_step = state.step;

        self.buf.clear();
        let buf = &mut self.buf;

        let step = state.steps[state.step];

        let max_len = state.steps.len();

        writeln!(buf, "fn brute_force(segments : &[Segment]) {{").unwrap();
        if step.is_init() {
            write!(buf, ">>").unwrap();
        } else {
            write!(buf, "  ").unwrap();
        }
        writeln!(buf, " // Starting").unwrap();
        writeln!(buf).unwrap();
        writeln!(buf, "  let len = segments.len() // {max_len}").unwrap();
        writeln!(buf).unwrap();
        writeln!(buf).unwrap();

        if let AlgorithmStep::Running {
            i,
            j,
            segment_i,
            segment_j,
            intersection,
            ..
        } = step
        {
            let segment1 = state.segments[segment_i];
            let segment2 = state.segments[segment_j];

            writeln!(buf, "  for i in 0..len {{ // i = {i}").unwrap();
            writeln!(buf, "    for j in i..len {{ // j = {j}").unwrap();
            writeln!(buf).unwrap();

            let s1_id = segment1.id;
            let s2_id = segment2.id;

            writeln!(buf, "      let segment1 = segments[i]; // Segment {s1_id}").unwrap();
            writeln!(buf, "      let segment2 = segments[j]; // Segment {s2_id}").unwrap();
            writeln!(buf).unwrap();

            if let Some(intersection) = intersection {
                let intersection = &state.intersections[intersection];
                let p1 = intersection.point1();
                let x = p1.x;
                let y = p1.y;
                writeln!(
                    buf,
                    "       // Intersection Point {{ x: {x:.2} y: {y:.2} }}",
                )
                .unwrap();
            } else {
                writeln!(buf, "       // No intersection").unwrap();
            }
        } else {
            writeln!(buf, "  for i in 0..len {{ // i = ").unwrap();
            writeln!(buf, "    for j in i..len {{ // j = ").unwrap();
            writeln!(buf).unwrap();
            writeln!(buf, "      let segment1 = segments[i]; // Segment ").unwrap();
            writeln!(buf, "      let segment2 = segments[j]; // Segment ").unwrap();
            writeln!(buf).unwrap();
            writeln!(buf, "       // Not yet calculated").unwrap();
        }
        writeln!(buf, "      let intersect = segment1.intersect(segment2);").unwrap();
        writeln!(buf).unwrap();
        writeln!(buf, "    }}").unwrap();
        writeln!(buf, "  }}").unwrap();
        writeln!(buf).unwrap();
        if step.is_end() {
            write!(buf, ">>").unwrap();
        } else {
            write!(buf, "  ").unwrap();
        }
        writeln!(buf, " // Finished").unwrap();
        writeln!(buf).unwrap();
        writeln!(buf, "}}").unwrap();
    }
}

fn show_code(ui: &mut egui::Ui, code: &str) {
    let code = remove_leading_indentation(code.trim_start_matches('\n'));
    rust_view_ui(ui, &code);
}

#[allow(clippy::trivially_copy_pass_by_ref, reason = "will be inlined anyway")]
#[inline(always)]
fn remove_leading_indentation(code: &str) -> String {
    const fn is_indent(c: &u8) -> bool {
        matches!(*c, b' ' | b'\t')
    }

    let first_line_indent = code.bytes().take_while(is_indent).count();

    let mut out = String::new();

    let mut code = code;
    while !code.is_empty() {
        let indent = code.bytes().take_while(is_indent).count();
        let start = first_line_indent.min(indent);
        let end = code
            .find('\n')
            .map_or_else(|| code.len(), |endline| endline + 1);
        out += &code[start..end];
        code = &code[end..];
    }
    out
}
pub(crate) fn rust_view_ui(ui: &mut egui::Ui, code: &str) {
    let language = "rs";
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    egui_extras::syntax_highlighting::highlight(ui.ctx(), ui.style(), &theme, code, language);
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, language);
}
