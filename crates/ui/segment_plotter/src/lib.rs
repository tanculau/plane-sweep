use core::f64::consts::TAU;

use common::{
    AlgoStepIdx, MyWidget, WidgetName,
    intersection::{IntersectionType, Intersections},
    segment::Segments,
};
use eframe::egui::{self, ComboBox, ScrollArea, TextWrapMode, remap};
use egui_plot::{CoordinatesFormatter, Corner, Legend, Line, LineStyle, Plot, PlotPoints};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(clippy::struct_excessive_bools, reason = "false positive")]
pub struct SegmentPlotter {
    proportional: bool,
    coordinates: bool,
    show_axes: bool,
    show_grid: bool,
    square: bool,
    line_style: LineStyle,
}

impl SegmentPlotter {
    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            proportional,
            coordinates,
            show_axes,
            show_grid,
            line_style,
            square,
            ..
        } = self;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.checkbox(show_axes, "Show axes");
                ui.checkbox(show_grid, "Show grid");
                ui.checkbox(coordinates, "Show coordinates on hover")
            });
            ui.vertical(|ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                ui.checkbox(square, "Square view")
                    .on_hover_text("Always keep the viewport square.");
                ui.checkbox(proportional, "Proportional data axes")
                    .on_hover_text("Tick are the same size on both axes.");
            });
            ComboBox::from_label("Line style")
                .selected_text(line_style.to_string())
                .show_ui(ui, |ui| {
                    for style in &[
                        LineStyle::Solid,
                        LineStyle::dashed_dense(),
                        LineStyle::dashed_loose(),
                        LineStyle::dotted_dense(),
                        LineStyle::dotted_loose(),
                    ] {
                        ui.selectable_value(line_style, *style, style.to_string());
                    }
                });
        });
    }
}

impl Default for SegmentPlotter {
    fn default() -> Self {
        Self {
            proportional: true,
            coordinates: true,
            show_axes: true,
            show_grid: true,
            square: false,
            line_style: LineStyle::Solid,
        }
    }
}

impl WidgetName for SegmentPlotter {
    const NAME: &'static str = "Segment Plotter";
}

impl<'segments, 'intersections> MyWidget<SegmentPlotterState<'segments, 'intersections>>
    for SegmentPlotter
{
    fn ui(
        &mut self,
        ui: &mut eframe::egui::Ui,
        state: impl Into<SegmentPlotterState<'segments, 'intersections>>,
    ) {
        let SegmentPlotterState {
            segments,
            intersections,
            step,
        } = state.into();
        ScrollArea::horizontal().show(ui, |ui| {
            self.options_ui(ui);
        });
        let mut plot = Plot::new("segment_plotter")
            .legend(Legend::default())
            .show_axes(self.show_axes)
            .show_grid(self.show_grid);
        if self.square {
            plot = plot.view_aspect(1.0);
        }
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        if self.coordinates {
            plot = plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());
        }
        plot.show(ui, |plot_ui| {
            for segment in segments {
                if segment.shown {
                    let mut line = Line::new(
                        format!(
                            "Segment {} {}",
                            segment.id,
                            if segment.mark { "(Active)" } else { "" }
                        ),
                        PlotPoints::new(vec![segment.upper.array(), segment.lower.array()]),
                    );
                    if segment.mark {
                        line = line.highlight(true);
                    }
                    plot_ui.line(line);
                }
            }
            for intersection in intersections
                .iter()
                .filter(|i| i.step().is_some_and(|s| AlgoStepIdx::from(s) <= step))
            {
                let name = format!(
                    "Intersection {} {}",
                    intersection.step().unwrap_or(0),
                    if intersection.mark() { "(Active)" } else { "" }
                );
                match intersection.typ() {
                    IntersectionType::Point { coord } => {
                        //plot_ui.points(Points::new(name, vec![[coord.x, coord.y]]));
                        let n = 512;
                        let circle_points: PlotPoints<'_> = (0..=n)
                            .map(|i| {
                                let t: f64 = remap(f64::from(i), 0.0..=f64::from(n), 0.0..=TAU);
                                let r: f64 = 1.0;
                                [r.mul_add(t.cos(), coord.x), r.mul_add(t.sin(), coord.y)]
                            })
                            .collect();
                        let line = Line::new(name, circle_points);
                        plot_ui.line(line);
                    }
                    IntersectionType::Parallel { line } => {
                        let mut line = Line::new(
                            name,
                            PlotPoints::new(vec![line.upper.array(), line.lower.array()]),
                        );
                        if intersection.mark() {
                            line = line.highlight(true);
                        }
                        plot_ui.line(line);
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SegmentPlotterState<'segments, 'intersections> {
    pub segments: &'segments Segments,
    pub intersections: &'intersections Intersections,
    pub step: AlgoStepIdx,
}
