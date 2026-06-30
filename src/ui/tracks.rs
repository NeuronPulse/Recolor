use crate::models::TrackNote;
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BORDER, TEXT_MUTED, TEXT_PRIMARY,
};

pub fn render_tracks(ui: &mut egui::Ui, notes: &[TrackNote]) {
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new("音轨编辑")
                .size(16.0)
                .color(TEXT_PRIMARY),
        );
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(format!("{} 个音符", notes.len()))
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Monospace),
        );
    });

    ui.add_space(12.0);
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
                egui::RichText::new("Track 1")
                    .size(11.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );
        });
        ui.add_space(8.0);

        ui.separator();
        ui.add_space(4.0);

        for (i, note) in notes.iter().enumerate() {
            render_note_row(ui, note, i);
        }

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

            for t in 0..=6 {
                ui.label(
                    egui::RichText::new(format!("{}s", t))
                        .size(11.0)
                        .color(TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );
                if t < 6 {
                    ui.add_space(40.0);
                }
            }
        });
    });
}

pub fn render_note_row(ui: &mut egui::Ui, note: &TrackNote, index: usize) {
    let row_frame = egui::Frame::new()
        .fill(egui::Color32::TRANSPARENT)
        .inner_margin(egui::Margin::symmetric(12, 4));

    row_frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{:02}", index + 1))
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
        });
    });
}
