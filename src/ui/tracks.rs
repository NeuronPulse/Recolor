use crate::models::{MidiData, TrackNote};
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BORDER, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

pub struct TracksResponse {
    pub import_clicked: bool,
}

pub fn render_tracks(
    ui: &mut egui::Ui,
    notes: &[TrackNote],
    midi_data: Option<&MidiData>,
    selected_track: &mut Option<usize>,
    error_message: Option<&String>,
) -> TracksResponse {
    let mut response = TracksResponse {
        import_clicked: false,
    };

    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new("音轨编辑")
                .size(16.0)
                .color(TEXT_PRIMARY),
        );
        ui.add_space(8.0);

        if let Some(data) = midi_data {
            ui.label(
                egui::RichText::new(format!("{} | {} BPM", data.file_name, data.tempo as u32))
                    .size(11.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );
        }
    });

    ui.add_space(12.0);

    if midi_data.is_none() {
        response.import_clicked = render_empty_state(ui, error_message);
        return response;
    }

    let data = midi_data.unwrap();

    ui.add_space(4.0);

    let track_selector_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 8));

    track_selector_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("轨道选择")
                    .size(12.0)
                    .color(TEXT_SECONDARY),
            );
            ui.add_space(12.0);

            let all_btn = egui::Button::new(
                egui::RichText::new("全部")
                    .size(11.0)
                    .color(if selected_track.is_none() {
                        egui::Color32::WHITE
                    } else {
                        TEXT_MUTED
                    }),
            )
            .corner_radius(4)
            .fill(if selected_track.is_none() {
                theme::ACCENT
            } else {
                egui::Color32::TRANSPARENT
            });

            if ui.add(all_btn).clicked() {
                *selected_track = None;
            }

            ui.add_space(8.0);

            for (idx, track) in data.tracks.iter().enumerate() {
                let is_selected = *selected_track == Some(idx);
                let btn = egui::Button::new(
                    egui::RichText::new(format!("{} ({} 音符)", track.name, track.note_count))
                        .size(11.0)
                        .color(if is_selected {
                            egui::Color32::WHITE
                        } else {
                            TEXT_MUTED
                        }),
                )
                .corner_radius(4)
                .fill(if is_selected {
                    theme::ACCENT
                } else {
                    egui::Color32::TRANSPARENT
                });

                if ui.add(btn).clicked() {
                    *selected_track = Some(idx);
                }

                ui.add_space(4.0);
            }
        });
    });

    ui.add_space(12.0);

    let track_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::same(0));

    track_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(format!("共 {} 个音符", notes.len()))
                    .size(11.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );
        });
        ui.add_space(8.0);

        ui.separator();
        ui.add_space(4.0);

        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            for (i, note) in notes.iter().enumerate() {
                render_note_row(ui, note, i);
            }
        });

        ui.add_space(8.0);
    });

    ui.add_space(12.0);

    let time_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 8));

    time_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("时间轴")
                    .size(11.0)
                    .color(TEXT_MUTED),
            );
            ui.add_space(12.0);

            let max_time = notes
                .iter()
                .map(|n| n.start + n.length)
                .fold(0.0f32, f32::max);
            let tick_count = (max_time.ceil() as u32).max(1).min(30);

            for t in 0..=tick_count {
                ui.label(
                    egui::RichText::new(format!("{}s", t))
                        .size(11.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
                if t < tick_count {
                    ui.add_space(40.0);
                }
            }
        });
    });

    response
}

fn render_empty_state(ui: &mut egui::Ui, error_message: Option<&String>) -> bool {
    let mut import_clicked = false;

    ui.add_space(40.0);

    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("🎵")
                .size(48.0)
                .color(TEXT_MUTED),
        );
        ui.add_space(16.0);
        ui.label(
            egui::RichText::new("未加载 MIDI 文件")
                .size(16.0)
                .color(TEXT_PRIMARY),
        );
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("请先导入 MIDI 文件以查看音轨")
                .size(12.0)
                .color(TEXT_MUTED),
        );
        ui.add_space(24.0);

        if let Some(err) = error_message {
            ui.label(
                egui::RichText::new(err)
                    .size(11.0)
                    .color(egui::Color32::from_rgb(220, 50, 50)),
            );
            ui.add_space(12.0);
        }

        let import_btn = egui::Button::new(
            egui::RichText::new("📂  导入 MIDI 文件")
                .size(13.0)
                .color(egui::Color32::WHITE),
        )
        .corner_radius(4)
        .fill(theme::ACCENT)
        .min_size(egui::vec2(160.0, 36.0));

        if ui.add(import_btn).clicked() {
            import_clicked = true;
        }
    });

    ui.add_space(40.0);

    import_clicked
}

fn render_note_row(ui: &mut egui::Ui, note: &TrackNote, index: usize) {
    let row_frame = egui::Frame::new()
        .fill(egui::Color32::TRANSPARENT)
        .inner_margin(egui::Margin::symmetric(12, 4));

    row_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{:03}", index + 1))
                    .size(11.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );
            ui.add_space(8.0);

            let pitch_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(3)
                .inner_margin(egui::Margin::symmetric(6, 2))
                .stroke(egui::Stroke::new(1.0, BORDER));

            pitch_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(&note.pitch)
                        .size(12.0)
                        .color(theme::MONO)
                        .family(egui::FontFamily::Monospace),
                );
            });

            ui.add_space(12.0);

            let note_label =
                format!("{:.2}s - {:.2}s", note.start, note.start + note.length);
            ui.label(
                egui::RichText::new(&note_label)
                    .size(10.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );

            ui.add_space(12.0);

            let vel_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(3)
                .inner_margin(egui::Margin::symmetric(6, 2))
                .stroke(egui::Stroke::new(1.0, BORDER));

            vel_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("vel:{}", note.velocity))
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
            });

            ui.add_space(8.0);

            let ch_frame = egui::Frame::new()
                .fill(BG_ELEVATED)
                .corner_radius(3)
                .inner_margin(egui::Margin::symmetric(6, 2))
                .stroke(egui::Stroke::new(1.0, BORDER));

            ch_frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("ch:{}", note.channel))
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
            });
        });
    });
}
