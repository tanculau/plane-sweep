use common::{segment::Segments, ui::MyWidget, ui::WidgetName};
use eframe::egui::{CentralPanel, SidePanel};

use crate::Step;

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetView;

pub struct SetViewState<'a, 'b> {
    pub step: &'a Step,
    pub segments: &'b Segments,
}

impl WidgetName for SetView {
    const NAME: &'static str = "Set View";
}

impl<'a, 'b> MyWidget<SetViewState<'a, 'b>> for SetView {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<SetViewState<'a, 'b>>) {
        let state = state.into();
        let step: &Step = state.step;
        let segments = state.segments;
        let u_p = &step.u_p;
        let c_p = &step.c_p;
        let l_p = &step.l_p;

        SidePanel::left("u_p")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("U(p)");
                    for idx in u_p {
                        let segment = segments[*idx].id;
                        ui.label(format!("s{segment}"));
                    }
                })
            });
        SidePanel::right("l_p")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("L(p)");
                    for idx in l_p {
                        let segment = segments[*idx].id;
                        ui.label(format!("s{segment}"));
                    }
                })
            });
        CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("C(p)");
                for idx in c_p {
                    let segment = segments[*idx].id;
                    ui.label(format!("s{segment}"));
                }
            })
        });
    }
}
