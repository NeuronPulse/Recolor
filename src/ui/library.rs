use crate::models::SampleItem;
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY,
};

pub fn render_library(ui: &mut egui::Ui, samples: &[SampleItem]) {
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new("素材库")
                .size(16.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Proportional),
        );
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(format!("{} 个切片", samples.len()))
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(12.0);
            let btn = egui::Button::new(
                egui::RichText::new("+ 导入素材").size(12.0).color(TEXT_PRIMARY),
            )
            .corner_radius(4)
            .fill(BG_ELEVATED)
            .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
            ui.add(btn);
        });
    });

    ui.add_space(8.0);
    ui.add_space(12.0);

    let header_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 6));

    header_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add_space(40.0 + 12.0 + 3.0 * 8.0);
            ui.label(egui::RichText::new("文件名").size(11.0).color(TEXT_MUTED));
            ui.add_space(80.0);
            ui.label(egui::RichText::new("时长").size(11.0).color(TEXT_MUTED));
            ui.add_space(40.0);
            ui.label(
                egui::RichText::new("采样率")
                    .size(11.0)
                    .color(TEXT_MUTED),
            );
            ui.add_space(40.0);
            ui.label(egui::RichText::new("格式").size(11.0).color(TEXT_MUTED));
            ui.add_space(40.0);
            ui.label(egui::RichText::new("音高").size(11.0).color(TEXT_MUTED));
        });
    });

    ui.add_space(4.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);
        ui.add_space(12.0);

        for (i, sample) in samples.iter().enumerate() {
            render_sample_item(ui, sample, i);
            ui.add_space(2.0);
        }
    });
}

pub fn render_sample_item(ui: &mut egui::Ui, sample: &SampleItem, _index: usize) {
    let item_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(4)
        .inner_margin(egui::Margin::symmetric(12, 8));

    item_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            let format_bg = egui::Color32::from_rgb(20, 20, 22);
            let format_frame = egui::Frame::new()
                .fill(format_bg)
                .corner_radius(4)
                .inner_margin(egui::Margin::same(6))
                .stroke(egui::Stroke::new(1.0, BORDER));

            format_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(&sample.format)
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
            });

            ui.add_space(4.0);

            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(&sample.name)
                        .size(13.0)
                        .color(TEXT_PRIMARY),
                );
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("时长: {}", sample.duration))
                            .size(11.0)
                            .color(TEXT_MUTED),
                    );
                    ui.label(
                        egui::RichText::new("·").size(11.0).color(TEXT_MUTED),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "采样率: {}",
                            sample.sample_rate
                        ))
                        .size(11.0)
                        .color(TEXT_MUTED),
                    );
                });
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let pitch_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(8, 3))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));

                pitch_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&sample.pitch)
                            .size(12.0)
                            .color(theme::MONO)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });
        });
    });
}
