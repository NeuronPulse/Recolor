use crate::core::project::Project;
use crate::theme::{self, BG_CARD, BG_ELEVATED, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY};

pub struct LibraryResponse {
    pub import_clicked: bool,
}

pub fn render_library(ui: &mut egui::Ui, project: &Project) -> LibraryResponse {
    let mut response = LibraryResponse {
        import_clicked: false,
    };

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
            egui::RichText::new(format!("{} 个切片", project.samples.len()))
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(12.0);
            let btn = egui::Button::new(
                egui::RichText::new("+ 导入素材")
                    .size(12.0)
                    .color(TEXT_PRIMARY),
            )
            .corner_radius(4)
            .fill(BG_ELEVATED)
            .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
            if ui.add(btn).clicked() {
                response.import_clicked = true;
            }
        });
    });

    ui.add_space(8.0);
    ui.add_space(12.0);

    if project.samples.is_empty() {
        render_empty_state(ui, &mut response.import_clicked);
        return response;
    }

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
            ui.label(egui::RichText::new("格式").size(11.0).color(TEXT_MUTED));
            ui.add_space(40.0);
            ui.label(egui::RichText::new("音高").size(11.0).color(TEXT_MUTED));
        });
    });

    ui.add_space(4.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);
        ui.add_space(12.0);

        for clip in &project.samples.clips {
            render_clip_item(ui, clip);
            ui.add_space(2.0);
        }
    });

    response
}

fn render_empty_state(ui: &mut egui::Ui, import_clicked: &mut bool) {
    ui.add_space(40.0);

    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("📂").size(48.0).color(TEXT_MUTED));
        ui.add_space(16.0);
        ui.label(
            egui::RichText::new("素材库为空")
                .size(16.0)
                .color(TEXT_PRIMARY),
        );
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("导入视频/音频素材目录")
                .size(12.0)
                .color(TEXT_MUTED),
        );
        ui.add_space(24.0);

        let import_btn = egui::Button::new(
            egui::RichText::new("📂  导入素材目录")
                .size(13.0)
                .color(egui::Color32::WHITE),
        )
        .corner_radius(4)
        .fill(theme::ACCENT)
        .min_size(egui::vec2(160.0, 36.0));

        if ui.add(import_btn).clicked() {
            *import_clicked = true;
        }
    });

    ui.add_space(40.0);
}

fn render_clip_item(ui: &mut egui::Ui, clip: &crate::core::sample::SampleClip) {
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
                    egui::RichText::new(&clip.format)
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
            });

            ui.add_space(4.0);

            ui.label(
                egui::RichText::new(&clip.name)
                    .size(13.0)
                    .color(TEXT_PRIMARY),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let pitch_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(8, 3))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));

                pitch_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(clip.pitch_name())
                            .size(12.0)
                            .color(theme::MONO)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });
        });
    });
}
