use super::veoveo_splash::VEOVEO_SPLASH;
use re_ui::{DesignTokens, UiExt as _};

pub(super) const DOCS_URL: &str = "https://www.rerun.io/docs";
pub(super) const WELCOME_SCREEN_TITLE: &str = "Visualize multimodal data";
pub(super) const WELCOME_SCREEN_BULLET_TEXT: &[&str] = &[
    "Log data with the Rerun SDK in C++, Python, or Rust",
    "Visualize and explore live or recorded data",
    "Configure the viewer interactively or through code",
];

/// Show the welcome section with VeoVeo splash.
pub(super) fn welcome_section_ui(ui: &mut egui::Ui) {
    // Just show the VeoVeo splash image
    let image = VEOVEO_SPLASH.as_image();
    ui.add(image);
}
