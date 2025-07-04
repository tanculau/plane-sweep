//! [`ThirdPartyLicences`] is a widget that displays all third party licenses used in the application.

use common::ui::{MyWidget, WidgetName};
use eframe::egui;

/// Displays all third party licenses used in the application.
#[derive(Debug, Default)]
pub struct ThirdPartyLicences {
    cache: egui_commonmark::CommonMarkCache,
}

impl MyWidget<()> for ThirdPartyLicences {
    #[allow(clippy::suboptimal_flops)]
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _state: impl Into<()>) {
        egui_commonmark::commonmark_str!(
            ui,
            &mut self.cache,
            "crates/ui/third_party_licenses/src/third_party_licences_generated.md"
        );
    }
}

impl WidgetName for ThirdPartyLicences {
    const NAME: &'static str = "Third Party Licences";
}

impl Clone for ThirdPartyLicences {
    fn clone(&self) -> Self {
        Self::default()
    }
}
