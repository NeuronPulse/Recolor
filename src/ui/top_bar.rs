use crate::theme::{self, BG_CARD, BG_ELEVATED, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY};

use super::state::Panel;

pub struct TopBarParams<'a> {
    pub active_panel: &'a mut Panel,
    pub project_name: &'a str,
    pub bpm: f32,
    pub is_playing: &'a mut bool,
    pub current_time: &'a mut f32,
    pub total_duration: f32,
    pub track_count: usize,
    pub sample_count: usize,
    pub stopped: &'a mut bool,
    pub play_toggled: &'a mut bool,
}

pub fn render_top_bar(ctx: &egui::Context, params: TopBarParams<'_>) {
    let TopBarParams {
        active_panel,
        project_name,
        bpm,
        is_playing,
        current_time,
        total_duration,
        track_count,
        sample_count,
        stopped,
        play_toggled,
    } = params;

    egui::TopBottomPanel::top("top_bar")
        .frame(
            egui::Frame::new()
                .fill(BG_CARD)
                .stroke(egui::Stroke::new(1.0, BORDER))
                .inner_margin(egui::Margin::symmetric(12, 6)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // ── Logo + Project Name ──
                ui.label(
                    egui::RichText::new("Re")
                        .size(15.0)
                        .color(theme::ACCENT)
                        .family(egui::FontFamily::Monospace),
                );
                ui.label(
                    egui::RichText::new("color")
                        .size(15.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );
                ui.add_space(8.0);

                let name_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(8, 3))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
                name_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(project_name)
                            .size(11.0)
                            .color(TEXT_PRIMARY)
                            .family(egui::FontFamily::Monospace),
                    );
                });

                ui.add_space(16.0);

                // ── Transport Controls ──
                let play_btn =
                    egui::Button::new(egui::RichText::new("▶").size(12.0).color(if *is_playing {
                        theme::ACCENT
                    } else {
                        TEXT_MUTED
                    }))
                    .corner_radius(3)
                    .fill(if *is_playing {
                        BG_ELEVATED
                    } else {
                        egui::Color32::TRANSPARENT
                    })
                    .min_size(egui::vec2(28.0, 22.0));
                if ui.add(play_btn).clicked() {
                    *is_playing = !*is_playing;
                    *play_toggled = true;
                }

                let stop_btn =
                    egui::Button::new(egui::RichText::new("■").size(12.0).color(TEXT_MUTED))
                        .corner_radius(3)
                        .fill(egui::Color32::TRANSPARENT)
                        .min_size(egui::vec2(28.0, 22.0));
                if ui.add(stop_btn).clicked() {
                    *is_playing = false;
                    *current_time = 0.0;
                    *stopped = true;
                }

                ui.add_space(12.0);

                // ── Time Display ──
                let time_str = format_time(*current_time);
                let total_str = format_time(total_duration);
                ui.label(
                    egui::RichText::new(format!("{} / {}", time_str, total_str))
                        .size(12.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );

                ui.add_space(16.0);

                // ── BPM ──
                ui.label(
                    egui::RichText::new("BPM")
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
                ui.add_space(4.0);
                let bpm_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(3)
                    .inner_margin(egui::Margin::symmetric(6, 3))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
                bpm_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", bpm as u32))
                            .size(12.0)
                            .color(TEXT_PRIMARY)
                            .family(egui::FontFamily::Monospace),
                    );
                });

                ui.add_space(12.0);

                // ── Stats ──
                ui.label(
                    egui::RichText::new(format!("{} 轨", track_count))
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(format!("{} 素材", sample_count))
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );

                // ── Right side: Panel tabs ──
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let tabs = [
                        (Panel::Generate, "生成"),
                        (Panel::Tracks, "编排"),
                        (Panel::Library, "素材库"),
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
                            egui::RichText::new(label).size(11.0).color(text_color),
                        )
                        .corner_radius(3)
                        .fill(bg)
                        .min_size(egui::vec2(52.0, 22.0));
                        if ui.add(btn).clicked() {
                            *active_panel = panel.clone();
                        }
                    }

                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("v0.1")
                            .size(10.0)
                            .color(TEXT_MUTED)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });
        });
}

fn format_time(seconds: f32) -> String {
    let total_secs = seconds.max(0.0);
    let hours = (total_secs / 3600.0) as u32;
    let mins = ((total_secs % 3600.0) / 60.0) as u32;
    let secs = (total_secs % 60.0) as u32;
    let cs = ((total_secs * 100.0) as u32) % 100;
    if hours > 0 {
        format!("{}:{:02}:{:02}.{:02}", hours, mins, secs, cs)
    } else {
        format!("{:02}:{:02}.{:02}", mins, secs, cs)
    }
}
