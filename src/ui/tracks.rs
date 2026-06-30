use crate::core::midi::{MidiData, MidiNote};
use crate::core::sample::SampleLibrary;
use crate::theme::{self, BG_CARD, BG_ELEVATED, BORDER, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY};

use super::piano_roll::{self, PianoRollState};

pub struct TracksResponse {
    pub import_clicked: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn render_tracks(
    ui: &mut egui::Ui,
    notes: &[MidiNote],
    midi_data: Option<&MidiData>,
    _samples: &SampleLibrary,
    selected_track: &mut Option<usize>,
    error_message: Option<&String>,
    piano_state: &mut PianoRollState,
    current_time: f32,
    is_playing: bool,
) -> TracksResponse {
    let mut response = TracksResponse {
        import_clicked: false,
    };

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(egui::RichText::new("编排").size(16.0).color(TEXT_PRIMARY));
        ui.add_space(8.0);

        if let Some(data) = midi_data {
            ui.label(
                egui::RichText::new(format!(
                    "{} | {} BPM | {} 音符",
                    data.file_name,
                    data.tempo as u32,
                    data.notes.len()
                ))
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
            );
        }
    });

    ui.add_space(8.0);

    if midi_data.is_none() {
        let roll_resp = piano_roll::render_piano_roll(
            ui,
            notes,
            false,
            error_message,
            piano_state,
            current_time,
            false,
            120.0,
        );
        response.import_clicked = roll_resp.import_clicked;
        return response;
    }

    let data = midi_data.unwrap();

    // ── Toolbar: track selector + zoom controls ──
    let toolbar_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 6));

    toolbar_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            // Track selector
            ui.label(egui::RichText::new("轨道").size(11.0).color(TEXT_SECONDARY));
            ui.add_space(6.0);

            let all_btn = egui::Button::new(egui::RichText::new("全部").size(10.0).color(
                if selected_track.is_none() {
                    egui::Color32::WHITE
                } else {
                    TEXT_MUTED
                },
            ))
            .corner_radius(3)
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
                        .size(10.0)
                        .color(if is_selected {
                            egui::Color32::WHITE
                        } else {
                            TEXT_MUTED
                        }),
                )
                .corner_radius(3)
                .fill(if is_selected {
                    theme::ACCENT
                } else {
                    egui::Color32::TRANSPARENT
                });

                if ui.add(btn).clicked() {
                    *selected_track = Some(idx);
                }

                ui.add_space(3.0);
            }

            // ── Zoom controls (right side) ──
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Horizontal zoom
                ui.label(
                    egui::RichText::new("缩放")
                        .size(10.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
                ui.add_space(4.0);

                let zoom_out_btn =
                    egui::Button::new(egui::RichText::new("−").size(12.0).color(TEXT_SECONDARY))
                        .corner_radius(3)
                        .fill(BG_ELEVATED)
                        .min_size(egui::vec2(22.0, 20.0));
                if ui.add(zoom_out_btn).clicked() {
                    piano_state.pixels_per_second =
                        (piano_state.pixels_per_second * 0.7).max(piano_roll::MIN_PPS);
                }

                let zoom_label = format!(
                    "{}%",
                    (piano_state.pixels_per_second / piano_roll::DEFAULT_PIXELS_PER_SECOND * 100.0)
                        as u32
                );
                ui.label(
                    egui::RichText::new(&zoom_label)
                        .size(10.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );

                let zoom_in_btn =
                    egui::Button::new(egui::RichText::new("+").size(12.0).color(TEXT_SECONDARY))
                        .corner_radius(3)
                        .fill(BG_ELEVATED)
                        .min_size(egui::vec2(22.0, 20.0));
                if ui.add(zoom_in_btn).clicked() {
                    piano_state.pixels_per_second =
                        (piano_state.pixels_per_second * 1.4).min(piano_roll::MAX_PPS);
                }

                ui.add_space(8.0);

                // Vertical zoom
                let v_out_btn =
                    egui::Button::new(egui::RichText::new("−").size(12.0).color(TEXT_SECONDARY))
                        .corner_radius(3)
                        .fill(BG_ELEVATED)
                        .min_size(egui::vec2(22.0, 20.0));
                if ui.add(v_out_btn).clicked() {
                    piano_state.row_height =
                        (piano_state.row_height * 0.7).max(piano_roll::MIN_ROW_HEIGHT);
                }

                let v_label = format!(
                    "行{}%",
                    (piano_state.row_height / piano_roll::DEFAULT_ROW_HEIGHT * 100.0) as u32
                );
                ui.label(
                    egui::RichText::new(&v_label)
                        .size(10.0)
                        .color(TEXT_PRIMARY)
                        .family(egui::FontFamily::Monospace),
                );

                let v_in_btn =
                    egui::Button::new(egui::RichText::new("+").size(12.0).color(TEXT_SECONDARY))
                        .corner_radius(3)
                        .fill(BG_ELEVATED)
                        .min_size(egui::vec2(22.0, 20.0));
                if ui.add(v_in_btn).clicked() {
                    piano_state.row_height =
                        (piano_state.row_height * 1.4).min(piano_roll::MAX_ROW_HEIGHT);
                }
            });
        });
    });

    ui.add_space(8.0);

    let roll_resp = piano_roll::render_piano_roll(
        ui,
        notes,
        true,
        error_message,
        piano_state,
        current_time,
        is_playing,
        data.tempo,
    );
    response.import_clicked = roll_resp.import_clicked;

    response
}
