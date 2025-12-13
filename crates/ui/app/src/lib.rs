// Based on https://github.com/emilk/eframe_template

use brute_force::ui::BruteForce;
use common::{
    MaybeSerde,
    math::{A, Float},
    ui::ToggleAbleWidget,
};
use eframe::egui;
use sweep::ui::PlaneSweep;
use voronoi::ui::Voron;

#[cfg(feature = "float")]
pub type AppFloat = App<common::math::Float>;

#[cfg(feature = "rational")]
pub type AppRational = App<common::math::Rational>;

#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App<T: A> {
    #[cfg_attr(feature = "serde", serde(skip))]
    third_party_licences: ToggleAbleWidget<third_party_licenses::ThirdPartyLicences, ()>,
    brute_force: BruteForce<T>,
    plane_sweep: PlaneSweep<T>,
    plane_sweep_with_lines: sweep_with_lines::ui::PlaneSweepOverlay<T>,
    selected: AlgorithmChoice,
    #[cfg_attr(feature = "serde", serde(skip))]
    tracing: ToggleAbleWidget<tracing_gui::Tracing, ()>,
    last_id: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    voronoi: Voron<Float>,
}

impl<T: A + for<'de> MaybeSerde<'de>> App<T> {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "serde")]
        {
            if let Some(storage) = cc.storage {
                let app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
                common::segment::set_counter(app.last_id);
                return app;
            }
        }
        let _ = cc;

        Self::default()
    }
}
impl<T: A + for<'de> MaybeSerde<'de>> eframe::App for App<T> {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.last_id = common::segment::get_counter();
        #[cfg(feature = "serde")]
        {
            eframe::set_value(storage, eframe::APP_KEY, &self);
        }
        let _ = storage;
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.separator();
                }
                ui.menu_button("Algorithmen", |ui| {
                    for &choice in AlgorithmChoice::CHOICES {
                        ui.radio_value(&mut self.selected, choice, choice.name());
                    }
                });
                ui.separator();
                ui.label("Theme:");
                egui::widgets::global_theme_preference_buttons(ui);
                ui.separator();

                ui.label("Zoom:");
                if ui.button("+").clicked() {
                    ctx.set_zoom_factor(ctx.zoom_factor() + 0.1);
                }
                if ui.button("-").clicked() {
                    ctx.set_zoom_factor(ctx.zoom_factor() - 0.1);
                }
                ui.separator();
                self.third_party_licences.add_toggle_value(ui);
                ui.separator();
                self.tracing.add_toggle_value(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected {
                AlgorithmChoice::None => {}
                AlgorithmChoice::PlaneSweepBruteForce => {
                    use common::ui::MyWidget;
                    self.brute_force.ui(ui, ());
                }
                AlgorithmChoice::PlaneSweep => {
                    use common::ui::MyWidget;
                    self.plane_sweep.ui(ui, ());
                }
                AlgorithmChoice::PlaneSweepWithLines => {
                    use common::ui::MyWidget;
                    self.plane_sweep_with_lines.ui(ui, ());
                }
                AlgorithmChoice::Voronoi => {
                    use common::ui::MyWidget;
                    self.voronoi.ui(ui, ());
                }
            }
            self.third_party_licences.view(ui.ctx(), ());
            self.tracing.view(ui.ctx(), ());
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum AlgorithmChoice {
    #[default]
    None,
    PlaneSweepBruteForce,
    PlaneSweep,
    PlaneSweepWithLines,
    Voronoi,
}

impl AlgorithmChoice {
    pub const CHOICES: &[Self] = &[
        Self::None,
        Self::PlaneSweepBruteForce,
        Self::PlaneSweep,
        Self::PlaneSweepWithLines,
        Self::Voronoi,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::PlaneSweepBruteForce => "Plane Sweep - Brute Force",
            Self::PlaneSweep => "Plane Sweep - Textbook",
            Self::PlaneSweepWithLines => "Plane Sweep - With Overlay",
            Self::Voronoi => "Voronoi",
        }
    }
}
