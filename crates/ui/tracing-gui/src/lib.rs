//! [`Tracing`] is a widget for displaying logs and tracing events in the UI. It also initializes the tracing subsystem to log all messages to stdout, if available, the this Widget, and the Web Console, if available.
use common::{MyWidget, WidgetName};
use egui_tracing::{EventCollector, tracing::collector::AllowedTargets};
use tracing::{error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]

/// Display and initialize logging.
///
/// [`Tracing`] is the Widget for displaying logs and tracing events in the UI.
/// It initializes the tracing subsystem to log all messages to stdout, if available,
/// Calling [`Tracing::default`] if a global default subscriber is already set does nothing and the Widget won't show any logs.
///
pub struct Tracing {
    collector: EventCollector,
}

impl Default for Tracing {
    fn default() -> Self {
        let mut res = Self {
            collector: EventCollector::default(),
        };
        res.collector = res.collector.allowed_targets(AllowedTargets::Selected(vec![
            "controller".to_string(),
            "init_tracing".to_string(),
            "segment_table".to_string(),
            "Segment::contains_coord".to_string(),
            "brute_force".to_string(),
        ]));
        res.init_tracing();
        res
    }
}

impl Tracing {
    #[must_use]
    pub fn collector(&self) -> EventCollector {
        self.collector.clone()
    }

    #[instrument(name = "init_tracing", skip(self))]
    pub fn init_tracing(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = tracing_subscriber::registry()
                .with(self.collector())
                .with(tracing_wasm::WASMLayer::default())
                .try_init()
                .inspect_err(|err| error!("Failed to initialize tracing: {}", err));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (tracing_subscriber::fmt()
                .compact()
                .finish()
                .with(self.collector()))
            .try_init()
            .inspect_err(|err| error!("Failed to initialize tracing: {}", err));
        }

        info!("Tracing initialized");
    }
}

impl WidgetName for Tracing {
    const NAME: &'static str = "Logs";
    const NAME_LONG: &'static str = "Logging Events";
}

impl MyWidget<()> for Tracing {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _state: impl Into<()>) {
        ui.add(egui_tracing::ui::Logs::new(self.collector()));
    }
}
