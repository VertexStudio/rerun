mod example_section;
mod loading_data_ui;
mod no_data_ui;
mod veoveo_splash;
mod welcome_section;

use std::sync::Arc;

use example_section::{ExampleSection, MIN_COLUMN_WIDTH};
use re_smart_channel::SmartChannelSource;
use veoveo_splash::veoveo_splash_ui;

use crate::app_state::WelcomeScreenState;

#[derive(Default)]
pub struct WelcomeScreen {
    example_page: ExampleSection,
}

impl WelcomeScreen {
    pub fn set_examples_manifest_url(&mut self, egui_ctx: &egui::Context, url: String) {
        self.example_page.set_manifest_url(egui_ctx, url);
    }

    /// Welcome screen shown in place of the viewport when no data is loaded.
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        welcome_screen_state: &WelcomeScreenState,
        log_sources: &[Arc<SmartChannelSource>],
    ) {
        if welcome_screen_state.opacity <= 0.0 {
            return;
        }

        // This is needed otherwise `example_page_ui` bleeds by a few pixels over the timeline panel
        // TODO(ab): figure out why that happens
        ui.set_clip_rect(ui.available_rect_before_wrap());

        let horizontal_scroll = ui.available_width() < 40.0 * 2.0 + MIN_COLUMN_WIDTH;

        let response = egui::ScrollArea::new([horizontal_scroll, true])
            .id_salt("welcome_screen_page")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Frame {
                    inner_margin: egui::Margin {
                        left: 40,
                        right: 40,
                        top: 50,
                        bottom: 8,
                    },
                    ..Default::default()
                }
                .show(ui, |ui| {
                    // Just show the VeoVeo splash - no examples, no other UI
                    veoveo_splash_ui(ui);
                });
            });

        if welcome_screen_state.opacity < 1.0 {
            let cover_opacity = 1.0 - welcome_screen_state.opacity;
            let fill_color = ui.visuals().panel_fill.gamma_multiply(cover_opacity);
            ui.painter()
                .rect_filled(response.inner_rect, 0.0, fill_color);
        }
    }
}
