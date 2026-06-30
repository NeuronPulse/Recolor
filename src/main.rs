mod app;
mod midi;
mod models;
mod theme;
mod ui;

use std::fs;

fn main() -> eframe::Result {
    let mut fonts = egui::FontDefinitions::default();

    if let Ok(data) = fs::read("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc") {
        fonts.font_data.insert(
            "noto_cjk_regular".to_owned(),
            egui::FontData::from_owned(data).into(),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "noto_cjk_regular".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "noto_cjk_regular".to_owned());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1080.0, 720.0])
            .with_min_inner_size([800.0, 520.0])
            .with_title("Recolor"),
        ..Default::default()
    };

    eframe::run_native(
        "Recolor",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_fonts(fonts);
            theme::apply_openai_style(&cc.egui_ctx);
            Ok(Box::new(app::App::new()))
        }),
    )
}
