use common::{AlgoStepIdx, AlgoSteps, intersection::Intersections, ui::MyWidget, ui::WidgetName};
use eframe::egui::{self, Window};
use tracing::{info, instrument};

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Controller {}

impl Controller {
    pub fn set_step(idx: &mut AlgoStepIdx, step: usize, steps: usize) {
        let step = step.clamp(0, steps.saturating_sub(1));
        let step = step.into();
        *idx = step;
    }

    pub fn reset(&mut self, step: &mut AlgoStepIdx) {
        *step = 0.into();
    }
}
impl WidgetName for Controller {
    const NAME: &'static str = "Controller";
    const NAME_LONG: &'static str = "Algorithm Controller";
}

#[derive(Debug)]
pub struct ControllerState<'a, 'b, 'c, T> {
    pub steps: &'a mut AlgoSteps<T>,
    pub step: &'b mut AlgoStepIdx,
    pub intersections: &'c mut Intersections,
}

impl<'a, 'b, 'c, T> MyWidget<ControllerState<'a, 'b, 'c, T>> for Controller {
    #[instrument(name = "Controller", skip_all)]
    fn ui(&mut self, ui: &mut eframe::egui::Ui, state: impl Into<ControllerState<'a, 'b, 'c, T>>) {
        let state = state.into();
        ui.vertical(|ui| {
            let text = match state.step.into() {
                0_usize => "Not started yet",
                r if r == state.steps.len() - 1 => "Finished",
                _ => "Running",
            };
            ui.label(text);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Step: ");
                let mut step: usize = state.step.into();
                let res = ui.add(egui::Slider::new(
                    &mut step,
                    0..=state.steps.len().saturating_sub(1),
                ));
                if res.changed() {
                    Self::set_step(state.step, step, state.steps.len());
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("⏹").clicked() {
                    info!("Go to step 0");
                    Self::set_step(state.step, 0, state.steps.len());
                }
                if ui.button("⏮").clicked() {
                    info!("Go to step 1");

                    Self::set_step(state.step, 1, state.steps.len());
                }
                if ui.button("⏴").clicked() {
                    info!("Go to prev step");
                    Self::set_step(
                        state.step,
                        usize::from(*state.step).saturating_sub(1),
                        state.steps.len(),
                    );
                }
                if ui.button("⏵").clicked() {
                    info!("Go to next step");

                    Self::set_step(state.step, usize::from(*state.step) + 1, state.steps.len());
                }
                if ui.button("⏭").clicked() {
                    info!("Go to last step");

                    Self::set_step(state.step, usize::MAX, state.steps.len());
                }
            });
        });
    }
    fn show(
        &mut self,
        ctx: &eframe::egui::Context,
        open: &mut bool,
        state: impl Into<ControllerState<'a, 'b, 'c, T>>,
    ) {
        Window::new(Self::NAME_LONG)
            .open(open)
            .resizable([true, false])
            .show(ctx, |ui| {
                self.ui(ui, state);
            });
    }
}
