use crate::models::Panel;
use crate::theme::{self, BG_CARD, BG_ELEVATED, BORDER, TEXT_MUTED, TEXT_PRIMARY};

pub fn render_top_bar(ctx: &egui::Context, active_panel: &mut Panel) {
    egui::TopBottomPanel::top("top_bar")
        .frame(
            egui::Frame::new()
                .fill(BG_CARD)
                .stroke(egui::Stroke::new(1.0, BORDER))
                .inner_margin(egui::Margin::symmetric(16, 8)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Re")
                        .size(16.0)
                        .color(theme::ACCENT)
                        .family(egui::FontFamily::Monospace),
                );
                ui.label(
                    egui::RichText::new("color")
                        .size(16.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );

                ui.add_space(24.0);

                let tabs = [
                    (Panel::Library, "素材库"),
                    (Panel::Tracks, "音轨"),
                    (Panel::Generate, "生成"),
                ];
                for (panel, label) in tabs {
                    let is_active = *active_panel == panel;
                    let text_color = if is_active { TEXT_PRIMARY } else { TEXT_MUTED };
                    let bg = if is_active {
                        BG_ELEVATED
                    } else {
                        egui::Color32::TRANSPARENT
                    };

                    let btn = egui::Button::new(
                        egui::RichText::new(label).size(13.0).color(text_color),
                    )
                    .min_size(egui::vec2(60.0, 28.0))
                    .corner_radius(4)
                    .fill(bg);
                    if ui.add(btn).clicked() {
                        *active_panel = panel.clone();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("v0.1.0")
                            .size(11.0)
                            .color(TEXT_MUTED)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });
        });
}
