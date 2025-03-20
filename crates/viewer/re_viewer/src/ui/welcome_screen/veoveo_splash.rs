use re_ui::Icon;

use re_ui::UiExt as _;

pub const VEOVEO_SPLASH: Icon = Icon::new(
    "veoveo_splash",
    include_bytes!("../../../data/veoveo_splash.png"),
);

/// Show a minimal welcome section with just a centered image.
pub fn veoveo_splash_ui(ui: &mut egui::Ui) -> bool {
    ui.center("veoveo_splash_contents", |ui| {
        let image = VEOVEO_SPLASH.as_image();
        ui.add(image);
    });
    true
}
