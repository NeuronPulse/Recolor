use crate::models::SampleItem;
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BG_HOVER, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY,
    TEXT_SECONDARY,
};

pub fn render_generate(
    ui: &mut egui::Ui,
    midi_name: &str,
    track_count: usize,
    samples: &[SampleItem],
    is_generating: &mut bool,
    generate_progress: &mut f32,
) {
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
                ui.label(
                    egui::RichText::new(midi_name)
                        .size(12.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );
            });
            ui.label(
                egui::RichText::new(format!("({} Tracks)", track_count))
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
                    egui::RichText::new(format!("{} 个切片已就绪", samples.len()))
                        .size(12.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );
            });
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("算法")
                    .size(12.0)
                    .color(TEXT_SECONDARY)
                    .family(egui::FontFamily::Monospace),
            );
            ui.add_space(8.0);
            let algo_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(4)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
            algo_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Pitch-Match v2")
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

        if *is_generating {
            ui.add_space(8.0);

            let progress_bg = egui::Color32::from_rgb(20, 20, 22);
            let progress_frame = egui::Frame::new()
                .fill(progress_bg)
                .corner_radius(4)
                .inner_margin(egui::Margin::same(0));

            progress_frame.show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                let available_w = ui.available_width();
                let (r, _) = ui.allocate_exact_size(
                    egui::vec2(available_w, 6.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(r, 3, BORDER);
                let fill_rect = egui::Rect::from_min_size(
                    r.min,
                    egui::vec2(r.width() * *generate_progress, r.height()),
                );
                ui.painter().rect_filled(fill_rect, 3, theme::ACCENT);
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("正在编排...")
                        .size(12.0)
                        .color(TEXT_SECONDARY),
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!("{}%", (*generate_progress * 100.0) as i32))
                        .size(12.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
            });
        } else {
            ui.label(
                egui::RichText::new("等待生成...")
                    .size(12.0)
                    .color(TEXT_MUTED),
            );
        }
    });

    ui.add_space(16.0);
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);

        let generate_btn = egui::Button::new(
            egui::RichText::new("⚡  一键编排并演奏")
                .size(13.0)
                .color(egui::Color32::WHITE)
                .family(egui::FontFamily::Proportional),
        )
        .corner_radius(4)
        .fill(if *is_generating { BG_HOVER } else { theme::ACCENT })
        .min_size(egui::vec2(180.0, 36.0));

        if ui
            .add_enabled(!*is_generating, generate_btn)
            .clicked()
        {
            *is_generating = true;
            *generate_progress = 0.0;
        }

        ui.add_space(8.0);

        let export_btn = egui::Button::new(
            egui::RichText::new("导出音频")
                .size(13.0)
                .color(TEXT_SECONDARY),
        )
        .corner_radius(4)
        .fill(BG_ELEVATED)
        .stroke(egui::Stroke::new(1.0, BORDER_LIGHT))
        .min_size(egui::vec2(100.0, 36.0));
        ui.add_enabled(false, export_btn);
    });
}
