use crate::models::{MidiData, TrackNote};
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BORDER, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY,
};

use super::piano_roll;

#[derive(Clone, PartialEq)]
pub enum TrackView {
    PianoRoll,
    List,
}

pub struct TracksResponse {
    pub import_clicked: bool,
}

pub fn render_tracks(
    ui: &mut egui::Ui,
    notes: &[TrackNote],
    midi_data: Option<&MidiData>,
    selected_track: &mut Option<usize>,
    view_mode: &mut TrackView,
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
                egui::RichText::new(format!("{} | {} BPM | {} 音符", data.file_name, data.tempo as u32, data.notes.len()))
                    .size(11.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );
        }
    });

    ui.add_space(12.0);

    if midi_data.is_none() {
        let roll_resp =
            piano_roll::render_piano_roll(ui, notes, false, error_message);
        response.import_clicked = roll_resp.import_clicked;
        return response;
    }

    let data = midi_data.unwrap();

    ui.add_space(4.0);

    let toolbar_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 8));

    toolbar_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("轨道")
                    .size(12.0)
                    .color(TEXT_SECONDARY),
            );
            ui.add_space(8.0);

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

            ui.add_space(4.0);

            for (idx, track) in data.tracks.iter().enumerate() {
                let is_selected = *selected_track == Some(idx);
                let btn = egui::Button::new(
                    egui::RichText::new(format!("{} ({})", track.name, track.note_count))
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

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let list_btn = egui::Button::new(
                    egui::RichText::new("☰ 列表")
                        .size(11.0)
                        .color(if *view_mode == TrackView::List {
                            egui::Color32::WHITE
                        } else {
                            TEXT_MUTED
                        }),
                )
                .corner_radius(4)
                .fill(if *view_mode == TrackView::List {
                    BG_ELEVATED
                } else {
                    egui::Color32::TRANSPARENT
                });

                if ui.add(list_btn).clicked() {
                    *view_mode = TrackView::List;
                }

                ui.add_space(4.0);

                let roll_btn = egui::Button::new(
                    egui::RichText::new("🎹 瀑布流")
                        .size(11.0)
                        .color(if *view_mode == TrackView::PianoRoll {
                            egui::Color32::WHITE
                        } else {
                            TEXT_MUTED
                        }),
                )
                .corner_radius(4)
                .fill(if *view_mode == TrackView::PianoRoll {
                    BG_ELEVATED
                } else {
                    egui::Color32::TRANSPARENT
                });

                if ui.add(roll_btn).clicked() {
                    *view_mode = TrackView::PianoRoll;
                }
            });
        });
    });

    ui.add_space(12.0);

    match view_mode {
        TrackView::PianoRoll => {
            let roll_resp =
                piano_roll::render_piano_roll(ui, notes, true, error_message);
            response.import_clicked = roll_resp.import_clicked;
        }
        TrackView::List => {
            render_note_list(ui, notes);
        }
    }

    response
}

fn render_note_list(ui: &mut egui::Ui, notes: &[TrackNote]) {
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

        egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(egui::RichText::new("#").size(10.0).color(TEXT_MUTED).family(egui::FontFamily::Monospace));
                ui.add_space(28.0);
                ui.label(egui::RichText::new("音高").size(10.0).color(TEXT_MUTED));
                ui.add_space(32.0);
                ui.label(egui::RichText::new("时间范围").size(10.0).color(TEXT_MUTED));
                ui.add_space(80.0);
                ui.label(egui::RichText::new("力度").size(10.0).color(TEXT_MUTED));
                ui.add_space(24.0);
                ui.label(egui::RichText::new("通道").size(10.0).color(TEXT_MUTED));
            });

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(2.0);

            for (i, note) in notes.iter().enumerate() {
                render_note_row(ui, note, i);
            }

            ui.add_space(4.0);
        });
    });
}

fn render_note_row(ui: &mut egui::Ui, note: &TrackNote, index: usize) {
    let row_frame = egui::Frame::new()
        .fill(egui::Color32::TRANSPARENT)
        .inner_margin(egui::Margin::symmetric(12, 3));

    row_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{:03}", index + 1))
                    .size(10.0)
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
                        .size(11.0)
                        .color(theme::MONO)
                        .family(egui::FontFamily::Monospace),
                );
            });

            ui.add_space(12.0);

            ui.label(
                egui::RichText::new(format!("{:.2}s – {:.2}s", note.start, note.start + note.length))
                    .size(10.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );

            ui.add_space(8.0);

            let vel_pct = note.velocity as f32 / 127.0;
            let vel_color = egui::Color32::from_rgb(
                (16.0 + vel_pct * 100.0) as u8,
                (163.0 - vel_pct * 40.0) as u8,
                (127.0 - vel_pct * 60.0) as u8,
            );
            ui.label(
                egui::RichText::new(format!("{:3}", note.velocity))
                    .size(10.0)
                    .color(vel_color)
                    .family(egui::FontFamily::Monospace),
            );

            ui.add_space(12.0);

            ui.label(
                egui::RichText::new(format!("ch:{:2}", note.channel))
                    .size(10.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );
        });
    });
}
