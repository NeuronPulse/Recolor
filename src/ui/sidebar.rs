use crate::models::{Panel, SampleItem};
use crate::theme::{self, BG_ELEVATED, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY};

pub fn render_sidebar(
    ui: &mut egui::Ui,
    active_panel: &mut Panel,
    midi_name: &str,
    track_count: usize,
    samples: &[SampleItem],
) {
    ui.add_space(8.0);

    ui.label(
        egui::RichText::new("导航")
            .size(11.0)
            .color(TEXT_MUTED)
            .family(egui::FontFamily::Proportional),
    );
    ui.add_space(4.0);

    let nav_items = [
        (Panel::Library, "📚", "素材库"),
        (Panel::Tracks, "🎵", "音轨编辑"),
        (Panel::Generate, "⚡", "一键生成"),
    ];

    for (panel, icon, label) in nav_items {
        let is_active = *active_panel == panel;
        let bg = if is_active {
            BG_ELEVATED
        } else {
            egui::Color32::TRANSPARENT
        };
        let text_c = if is_active { TEXT_PRIMARY } else { TEXT_SECONDARY };
        let indicator = if is_active {
            theme::ACCENT
        } else {
            egui::Color32::TRANSPARENT
        };

        let btn = egui::Button::new(
            egui::RichText::new(format!("{}  {}", icon, label))
                .size(13.0)
                .color(text_c),
        )
        .min_size(egui::vec2(ui.available_width(), 32.0))
        .corner_radius(4)
        .fill(bg);

        let response = ui.add(btn);
        if response.clicked() {
            *active_panel = panel.clone();
        }
        if is_active {
            let rect = response.rect;
            ui.painter().line_segment(
                [rect.left_top(), rect.left_bottom()],
                egui::Stroke::new(2.0, indicator),
            );
        }
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label(egui::RichText::new("项目信息").size(11.0).color(TEXT_MUTED));
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("MIDI")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
        ui.label(
            egui::RichText::new(midi_name)
                .size(11.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Monospace),
        );
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("轨数")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
        ui.label(
            egui::RichText::new(format!("{}", track_count))
                .size(11.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Monospace),
        );
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("素材")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
        ui.label(
            egui::RichText::new(format!("{} 个", samples.len()))
                .size(11.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Monospace),
        );
    });
}
