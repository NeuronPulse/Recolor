use crate::core::project::Project;
use crate::theme::{self, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY};

use super::state::Panel;

pub fn render_sidebar(ui: &mut egui::Ui, active_panel: &mut Panel, project: &Project) {
    // ── Browser Header ──
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("浏览器")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Proportional),
        );
    });
    ui.add_space(6.0);

    // ── Quick Actions ──
    render_section_header(ui, "快速操作");

    let actions = [
        ("📂", "导入 MIDI", Panel::Tracks),
        ("📁", "导入素材", Panel::Library),
    ];

    for (icon, label, target_panel) in actions {
        let btn = egui::Button::new(
            egui::RichText::new(format!("{}  {}", icon, label))
                .size(12.0)
                .color(TEXT_SECONDARY),
        )
        .min_size(egui::vec2(ui.available_width(), 28.0))
        .corner_radius(4)
        .fill(egui::Color32::TRANSPARENT);

        if ui.add(btn).clicked() {
            *active_panel = target_panel.clone();
        }
    }

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Project ──
    render_section_header(ui, "项目");

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("名称")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
        ui.label(
            egui::RichText::new(&project.name)
                .size(11.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Monospace),
        );
    });
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("BPM ")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
        ui.label(
            egui::RichText::new(format!("{}", project.bpm as u32))
                .size(11.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Monospace),
        );
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // ── MIDI File ──
    render_section_header(ui, "MIDI");

    if let Some(ref midi) = project.midi {
        render_info_row(ui, "文件", &midi.file_name);
        render_info_row(ui, "轨道", &format!("{}", midi.tracks.len()));
        render_info_row(ui, "音符", &format!("{}", midi.notes.len()));
    } else {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("未加载").size(11.0).color(TEXT_MUTED));
        });
    }

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Samples ──
    render_section_header(ui, "素材库");

    if project.samples.is_empty() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("无素材").size(11.0).color(TEXT_MUTED));
        });
    } else {
        render_info_row(ui, "切片", &format!("{} 个", project.samples.len()));

        // show first few samples
        let show_count = project.samples.len().min(8);
        for clip in project.samples.clips.iter().take(show_count) {
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                let dot_color = theme::ACCENT;
                ui.painter()
                    .circle_filled(ui.cursor().min + egui::vec2(3.0, 8.0), 2.0, dot_color);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&clip.name)
                        .size(10.0)
                        .color(TEXT_SECONDARY)
                        .family(egui::FontFamily::Monospace),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(clip.pitch_name())
                            .size(10.0)
                            .color(TEXT_MUTED)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });
        }
        if project.samples.len() > show_count {
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new(format!(
                        "... 还有 {} 个",
                        project.samples.len() - show_count
                    ))
                    .size(10.0)
                    .color(TEXT_MUTED),
                );
            });
        }
    }

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Arrangement ──
    render_section_header(ui, "编排");

    let clip_count = project
        .arrangement
        .tracks
        .iter()
        .map(|t| t.clips.len())
        .sum::<usize>();
    render_info_row(ui, "片段", &format!("{} 个", clip_count));

    if clip_count == 0 {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("未编排").size(10.0).color(TEXT_MUTED));
        });
    }

    ui.add_space(12.0);
}

fn render_section_header(ui: &mut egui::Ui, title: &str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(title)
                .size(10.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Proportional),
        );
    });
    ui.add_space(4.0);
}

fn render_info_row(ui: &mut egui::Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(format!("{} ", key))
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
        ui.label(
            egui::RichText::new(value)
                .size(11.0)
                .color(TEXT_PRIMARY)
                .family(egui::FontFamily::Monospace),
        );
    });
}
