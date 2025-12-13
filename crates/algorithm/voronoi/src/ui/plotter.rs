// Based on  https://github.com/emilk/egui_plot/tree/main/examples

use core::{f64::consts::TAU, fmt::Debug};

use common::{
    AlgoStepIdx, AlgoSteps,
    math::{B, Float},
    ui::{MyWidget, WidgetName},
};
use eframe::egui::{self, Color32, ComboBox, DragValue, ScrollArea, TextWrapMode, remap};
use egui_plot::{
    CoordinatesFormatter, Corner, HLine, Legend, Line, LineStyle, Plot, PlotPoints, Polygon,
};
use itertools::Itertools;

use crate::{BoundingBox, Dcel, Site, event_queue::Event, ui::steps::Step};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(clippy::struct_excessive_bools, reason = "false positive")]
pub struct VoronoiPlotter {
    proportional: bool,
    coordinates: bool,
    show_axes: bool,
    show_grid: bool,
    square: bool,
    line_style: LineStyle,
    radius: f64,
}

impl VoronoiPlotter {
    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            proportional,
            coordinates,
            show_axes,
            show_grid,
            line_style,
            square,
            radius,
            ..
        } = self;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Radius Site:");
                    ui.add(DragValue::new(radius).range(0.01..=100.0));
                });
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

// SOURCE https://github.com/emilk/egui_plot/blob/ac2d7cb54554cbdd87e298bd324471c9498f6a42/examples/lines/src/app.rs#L115
#[allow(clippy::many_single_char_names)]
fn circle(radius: f64, name: String, x: impl Into<f64>, y: impl Into<f64>) -> Polygon<'static> {
    let n = 512;
    let x = x.into();
    let y = y.into();
    let circle_points: PlotPoints<'static> = (0..=n)
        .map(|i| {
            let t: f64 = remap(f64::from(i), 0.0..=f64::from(n), 0.0..=TAU);
            let r: f64 = radius;
            [r.mul_add(t.cos(), x), r.mul_add(t.sin(), y)]
        })
        .collect();

    Polygon::new(name, circle_points)
}
// END SOURCE https://github.com/emilk/egui_plot/blob/ac2d7cb54554cbdd87e298bd324471c9498f6a42/examples/lines/src/app.rs#L115

impl Default for VoronoiPlotter {
    fn default() -> Self {
        Self {
            radius: 0.15,
            proportional: true,
            coordinates: true,
            show_axes: true,
            show_grid: true,
            square: false,
            line_style: LineStyle::Solid,
        }
    }
}

impl WidgetName for VoronoiPlotter {
    const NAME: &'static str = "Voronoi Plotter";
}

impl<'segments, 'intersections, 'steps, T: B>
    MyWidget<VoronoiPlotterState<'segments, 'intersections, 'steps, T>> for VoronoiPlotter
{
    #[allow(clippy::too_many_lines)]
    fn ui(
        &mut self,
        ui: &mut eframe::egui::Ui,
        state: impl Into<VoronoiPlotterState<'segments, 'intersections, 'steps, T>>,
    ) {
        let VoronoiPlotterState {
            sites,
            dcel: _,
            step,
            steps,
            stopped: _,
            bound,
        } = state.into();
        ScrollArea::horizontal().show(ui, |ui| {
            self.options_ui(ui);
        });
        let mut plot = Plot::new("site_plotter")
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
            // Sites
            for site in sites {
                let name = format!("Site {}", site.id);
                let line = circle(self.radius, name, site.x.to_float(), site.y.to_float());
                plot_ui.polygon(line);
            }

            // Bounding Box
            let line = Line::new(
                "BoundUp",
                PlotPoints::new(vec![
                    [bound.x_min.to_float(), bound.y_max.to_float()],
                    [bound.x_max.to_float(), bound.y_max.to_float()],
                ]),
            );
            plot_ui.line(line);
            let line = Line::new(
                "BoundDown",
                PlotPoints::new(vec![
                    [bound.x_min.to_float(), bound.y_min.to_float()],
                    [bound.x_max.to_float(), bound.y_min.to_float()],
                ]),
            );
            plot_ui.line(line);
            let line = Line::new(
                "BoundLeft",
                PlotPoints::new(vec![
                    [bound.x_min.to_float(), bound.y_min.to_float()],
                    [bound.x_min.to_float(), bound.y_max.to_float()],
                ]),
            );
            plot_ui.line(line);
            let line = Line::new(
                "BoundLeft",
                PlotPoints::new(vec![
                    [bound.x_max.to_float(), bound.y_min.to_float()],
                    [bound.x_max.to_float(), bound.y_max.to_float()],
                ]),
            );
            plot_ui.line(line);

            // Draw Arcs
            if let Some(event) = steps[step].event {
                let beach = &steps[step].beachline;
                let mut sites = vec![];
                beach.sites2(&mut sites);
                let y = event.corrected_y();

                for site in sites.into_iter().unique_by(|s| s.id) {
                    if site.y != y {
                        let line = arc(
                            format!("Arc {}", site.id),
                            site.x.to_float(),
                            site.y.to_float(),
                            y.to_float(),
                        );
                        plot_ui.line(line);
                    }
                }

                //let mut arcs = vec![];

                //if let Some(first) = sites.first() {
                //    arcs.push((first.x, first.y));
                //}

                // let mut tmp = sites.windows(2);
                // while let Some([l, r]) = tmp.next() {
                //     let breakpoint = breakpoint_between(l.x, l.y, r.x, r.y, y);
                //     let y = calc(l.x, l.y, r.x, r.y, breakpoint, y);
                //     arcs.push((breakpoint, y));
                // }
                //
                // if let Some(last) = sites.last() && sites.len() > 1 {
                //     arcs.push((last.x, last.y));
                // }

                //let mut tmp1 = arcs.windows(2).enumerate();
                //while let Some((idx, [l,r])) = tmp1.next() {
                //    let line = arc(format!("Arc {idx}"), l.0.to_float(), l.1.to_float(), r.0.to_float(), r.1.to_float(), y.to_float());
                //    plot_ui.line(line);
                //}
            }

            // Draw unbounded Halfedge pairs

            // Draw circles

            if let Some(Event::Circle(id, c)) = steps[step].event {
                draw_circle(plot_ui, c, format!("Circle {id:?} (Current)"));
            }

            let mut events = steps[step].event_queue.clone();
            while let Some(Event::Circle(id, c)) = events.pop() {
                draw_circle(plot_ui, c, format!("Circle {id:?}"));
            }

            // Sweep
            if let Some(event) = steps[step].event {
                let (x, y) = match event {
                    Event::Site(site) => (site.x.to_float(), site.y.to_float()),
                    Event::Circle(_, circle) => {
                        (circle.x.to_float(), (circle.y - circle.r).to_float())
                    }
                };

                plot_ui.hline(HLine::new("Sweep Line", y).color(Color32::RED));
                let line = circle(self.radius * 2.0, "Event Point".to_string(), x, y)
                    .fill_color(Color32::RED);
                plot_ui.polygon(line);
            }

            // Found edges
            for (id, [v1, v2]) in steps[step].dcel.get_all_vertices().enumerate() {
                let points = PlotPoints::new(vec![
                    [v1.x.to_float(), v1.y.to_float()],
                    [v2.x.to_float(), v2.y.to_float()],
                ]);
                let polygon = Line::new(format!("Line{id}"), points);
                plot_ui.line(polygon);
            }
        });
    }
}

fn draw_circle<T: B>(plot_ui: &mut egui_plot::PlotUi<'_>, c: crate::Circle<T>, name: String) {
    let circle = circle(c.r.to_float(), name, c.x.to_float(), c.y.to_float());
    plot_ui.polygon(circle);
}

fn arc(name: String, x1: f64, y1: f64, s: f64) -> Line<'static> {
    let circle_points: PlotPoints<'static> =
        PlotPoints::from_explicit_callback(move |i| calc2(x1, y1, i, s), .., 512);

    Line::new(name, circle_points)
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct VoronoiPlotterState<'sites, 'dcel, 'steps, T: B> {
    pub sites: &'sites [Site<T>],
    pub dcel: &'dcel Dcel<T>,
    pub bound: BoundingBox<T>,
    pub step: AlgoStepIdx,
    pub steps: &'steps AlgoSteps<Step<T>>,
    pub stopped: bool,
}

// https://www.wolframalpha.com/input?i=solve+%28x1+-+h%29%5E2+%2B+%28y1-k%29%5E2+%3D+%28k+-+s%29%5E2+for+k
// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
#[allow(clippy::suboptimal_flops)]
fn calc2(x1: f64, y1: f64, h: f64, s: f64) -> f64 {
    let a = h * h - f64::from(2) * h * x1 - s * s + x1 * x1 + y1 * y1;
    let b = f64::from(2) * s - f64::from(2) * y1;
    if Float::from(b) == Float::from(0.0) {
        s
    } else {
        -a / b
    }
}
