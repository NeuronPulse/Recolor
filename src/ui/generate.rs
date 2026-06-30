use crate::core::project::Project;
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

use super::state::ViewState;

pub fn render_generate(ui: &mut egui::Ui, project: &Project, _view: &ViewState) {
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new("一键生成")
                .size(16.0)
                .color(TEXT_PRIMARY),
        );
    });

    ui.add_space(16.0);
    ui.add_space(12.0);

    let config_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::same(16));

    config_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.label(
            egui::RichText::new("生成配置")
                .size(13.0)
                .color(TEXT_SECONDARY),
        );
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("MIDI")
                    .size(12.0)
                    .color(TEXT_SECONDARY)
                    .family(egui::FontFamily::Monospace),
            );
            ui.add_space(8.0);
            let midi_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(4)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
            midi_frame.show(ui, |ui| {
                let midi_name = project
                    .midi
                    .as_ref()
                    .map(|m| m.file_name.as_str())
                    .unwrap_or("未加载");
                ui.label(
                    egui::RichText::new(midi_name)
                        .size(12.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );
            });
            ui.label(
                egui::RichText::new(format!("({} Tracks)", project.track_count()))
                    .size(11.0)
                    .color(TEXT_MUTED),
            );
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("素材")
                    .size(12.0)
                    .color(TEXT_SECONDARY)
                    .family(egui::FontFamily::Monospace),
            );
            ui.add_space(8.0);
            let sample_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(4)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
            sample_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("{} 个切片已就绪", project.samples.len()))
                        .size(12.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("BPM")
                    .size(12.0)
                    .color(TEXT_SECONDARY)
                    .family(egui::FontFamily::Monospace),
            );
            ui.add_space(8.0);
            let bpm_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(4)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
            bpm_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("{}", project.bpm as u32))
                        .size(12.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );
            });
        });
    });

    ui.add_space(16.0);
    ui.add_space(12.0);

    let result_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::same(16));

    result_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.label(
            egui::RichText::new("输出预览")
                .size(13.0)
                .color(TEXT_SECONDARY),
        );
        ui.add_space(12.0);

        ui.label(
            egui::RichText::new("等待生成...")
                .size(12.0)
                .color(TEXT_MUTED),
        );
    });

    ui.add_space(16.0);
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);

        let can_generate = project.is_loaded() && !project.samples.is_empty();

        let generate_btn = egui::Button::new(
            egui::RichText::new("⚡  一键编排并生成")
                .size(13.0)
                .color(if can_generate {
                    egui::Color32::WHITE
                } else {
                    TEXT_MUTED
                })
                .family(egui::FontFamily::Proportional),
        )
        .corner_radius(4)
        .fill(if can_generate {
            theme::ACCENT
        } else {
            BG_ELEVATED
        })
        .min_size(egui::vec2(180.0, 36.0));

        ui.add_enabled(can_generate, generate_btn);

        ui.add_space(8.0);

        let export_btn =
            egui::Button::new(egui::RichText::new("导出视频").size(13.0).color(TEXT_MUTED))
                .corner_radius(4)
                .fill(BG_ELEVATED)
                .stroke(egui::Stroke::new(1.0, BORDER_LIGHT))
                .min_size(egui::vec2(100.0, 36.0));
        ui.add_enabled(false, export_btn);
    });
}
