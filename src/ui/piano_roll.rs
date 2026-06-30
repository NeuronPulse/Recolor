use egui::{Color32, Pos2, Rect, Vec2};

use crate::models::TrackNote;
use crate::theme::{
    self, BG_CARD, BG_ELEVATED, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY,
};

const ROW_HEIGHT: f32 = 18.0;
const NOTE_HEIGHT: f32 = 14.0;
const NOTE_Y_PAD: f32 = (ROW_HEIGHT - NOTE_HEIGHT) / 2.0;
const PIXELS_PER_SECOND: f32 = 120.0;
const KEY_LABEL_WIDTH: f32 = 48.0;
const HEADER_HEIGHT: f32 = 28.0;
const MIN_KEY: u8 = 24;
const MAX_KEY: u8 = 96;

const TRACK_COLORS: [Color32; 8] = [
    Color32::from_rgb(88, 166, 255),
    Color32::from_rgb(255, 136, 88),
    Color32::from_rgb(120, 220, 130),
    Color32::from_rgb(255, 200, 80),
    Color32::from_rgb(200, 130, 255),
    Color32::from_rgb(255, 130, 180),
    Color32::from_rgb(100, 210, 220),
    Color32::from_rgb(220, 180, 120),
];

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub struct PianoRollResponse {
    pub import_clicked: bool,
}

pub fn render_piano_roll(
    ui: &mut egui::Ui,
    notes: &[TrackNote],
    has_midi: bool,
    error_message: Option<&String>,
) -> PianoRollResponse {
    let mut response = PianoRollResponse {
        import_clicked: false,
    };

    if !has_midi {
        response.import_clicked = render_empty_state(ui, error_message);
        return response;
    }

    let total_height = (MAX_KEY - MIN_KEY + 1) as f32 * ROW_HEIGHT;
    let available = ui.available_size();
    let height = total_height.min(available.y - 40.0).max(200.0);
    let content_width = available.x.max(400.0);
    let total_width = content_width + KEY_LABEL_WIDTH;

    let outer_frame = egui::Frame::new()
        .fill(BG_CARD)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6)
        .inner_margin(egui::Margin::same(0));

    outer_frame.show(ui, |ui| {
        egui::ScrollArea::both()
            .max_height(height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let (_response, painter) = ui.allocate_painter(
                    Vec2::new(total_width, total_height),
                    egui::Sense::hover(),
                );

                let rect = _response.rect;
                let offset = Vec2::new(
                    ui.clip_rect().left() - rect.left(),
                    ui.clip_rect().top() - rect.top(),
                );

                draw_background(&painter, rect);
                draw_grid(&painter, rect, offset);
                draw_notes(&painter, rect, offset, notes);
                draw_piano_keys(&painter, rect, offset);
                draw_time_ruler(&painter, rect, offset);
                draw_header_border(&painter, rect);
            });
    });

    response
}

fn draw_background(painter: &egui::Painter, rect: Rect) {
    painter.rect_filled(rect, 0, BG_CARD);
}

fn draw_grid(painter: &egui::Painter, rect: Rect, offset: Vec2) {
    for key in MIN_KEY..=MAX_KEY {
        let y = (key - MIN_KEY) as f32 * ROW_HEIGHT - offset.y;
        if y + ROW_HEIGHT < rect.top() || y > rect.bottom() {
            continue;
        }

        let note_idx = (key % 12) as usize;
        let is_black = matches!(note_idx, 1 | 3 | 6 | 8 | 10);

        let row_color = if is_black {
            Color32::from_rgb(22, 22, 24)
        } else {
            Color32::from_rgb(28, 28, 30)
        };

        let row_rect = Rect::from_min_max(
            Pos2::new(rect.left(), rect.top() + y),
            Pos2::new(rect.right(), rect.top() + y + ROW_HEIGHT),
        );
        painter.rect_filled(row_rect, 0, row_color);

        painter.line_segment(
            [row_rect.left_top(), row_rect.right_top()],
            egui::Stroke::new(0.5, BORDER),
        );
    }

    let beat_duration = 60.0 / 120.0;
    let measure_duration = beat_duration * 4.0;
    let time_start = offset.x / PIXELS_PER_SECOND;
    let time_end = (offset.x + rect.width()) / PIXELS_PER_SECOND;

    let first_measure = (time_start / measure_duration).floor() as i32;
    let last_measure = (time_end / measure_duration).ceil() as i32;

    for m in first_measure..=last_measure {
        let t = m as f32 * measure_duration;
        let x = rect.left() + (t * PIXELS_PER_SECOND) - offset.x;
        if x >= rect.left() && x <= rect.right() {
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                egui::Stroke::new(1.0, BORDER_LIGHT),
            );
        }

        for beat in 1..4 {
            let bt = t + beat as f32 * beat_duration;
            let bx = rect.left() + (bt * PIXELS_PER_SECOND) - offset.x;
            if bx >= rect.left() && bx <= rect.right() {
                painter.line_segment(
                    [Pos2::new(bx, rect.top()), Pos2::new(bx, rect.bottom())],
                    egui::Stroke::new(0.5, BORDER),
                );
            }
        }
    }
}

fn draw_notes(painter: &egui::Painter, rect: Rect, offset: Vec2, notes: &[TrackNote]) {
    let time_start = offset.x / PIXELS_PER_SECOND;
    let time_end = (offset.x + rect.width()) / PIXELS_PER_SECOND;

    for note in notes {
        if note.key < MIN_KEY || note.key > MAX_KEY {
            continue;
        }

        let note_end = note.start + note.length;
        if note_end < time_start || note.start > time_end {
            continue;
        }

        let y =
            rect.top() + (note.key - MIN_KEY) as f32 * ROW_HEIGHT + NOTE_Y_PAD - offset.y;
        if y + NOTE_HEIGHT < rect.top() || y > rect.bottom() {
            continue;
        }

        let x = rect.left() + (note.start * PIXELS_PER_SECOND) - offset.x;
        let w = (note.length * PIXELS_PER_SECOND).max(3.0);

        let note_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, NOTE_HEIGHT));

        let base_color = TRACK_COLORS[note.track_index % TRACK_COLORS.len()];
        let alpha = (note.velocity as f32 / 127.0 * 180.0 + 75.0) as u8;
        let note_color = Color32::from_rgba_premultiplied(
            base_color.r(),
            base_color.g(),
            base_color.b(),
            alpha,
        );

        let clamped = note_rect.intersect(rect);
        if clamped.width() <= 0.0 || clamped.height() <= 0.0 {
            continue;
        }

        painter.rect_filled(clamped, 2.0, note_color);

        let border_color = Color32::from_rgba_premultiplied(
            base_color.r(),
            base_color.g(),
            base_color.b(),
            (alpha / 2).max(20),
        );
        painter.rect_stroke(
            clamped,
            2.0,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        if w > 28.0 {
            let text_color = Color32::from_rgba_premultiplied(255, 255, 255, 200);
            painter.text(
                clamped.left_center() + Vec2::new(4.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &note.pitch,
                egui::FontId::proportional(9.0),
                text_color,
            );
        }
    }
}

fn draw_piano_keys(painter: &egui::Painter, rect: Rect, offset: Vec2) {
    let key_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top()),
        Pos2::new(rect.left() + KEY_LABEL_WIDTH, rect.bottom()),
    );
    painter.rect_filled(key_rect, 0, BG_ELEVATED);
    painter.rect_stroke(
        key_rect,
        0,
        egui::Stroke::new(1.0, BORDER),
        egui::StrokeKind::Inside,
    );

    for key in MIN_KEY..=MAX_KEY {
        let y = (key - MIN_KEY) as f32 * ROW_HEIGHT - offset.y;
        if y + ROW_HEIGHT < rect.top() || y > rect.bottom() {
            continue;
        }

        let note_idx = (key % 12) as usize;
        let is_black = matches!(note_idx, 1 | 3 | 6 | 8 | 10);
        let octave = (key / 12) as i8 - 1;

        let row_rect = Rect::from_min_max(
            Pos2::new(rect.left(), rect.top() + y),
            Pos2::new(rect.left() + KEY_LABEL_WIDTH, rect.top() + y + ROW_HEIGHT),
        );

        let bg = if is_black {
            Color32::from_rgb(35, 35, 38)
        } else {
            BG_ELEVATED
        };
        painter.rect_filled(row_rect, 0, bg);

        let show_name = matches!(note_idx, 0 | 2 | 4 | 5 | 7 | 9 | 11);

        if show_name {
            let label = format!("{}{}", NOTE_NAMES[note_idx], octave);
            let text_color = if is_black { TEXT_MUTED } else { TEXT_PRIMARY };
            painter.text(
                row_rect.left_center() + Vec2::new(6.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &label,
                egui::FontId::proportional(9.0),
                text_color,
            );
        }

        painter.line_segment(
            [row_rect.left_bottom(), row_rect.right_bottom()],
            egui::Stroke::new(0.5, BORDER),
        );
    }
}

fn draw_time_ruler(painter: &egui::Painter, rect: Rect, offset: Vec2) {
    let ruler_rect = Rect::from_min_max(
        Pos2::new(rect.left() + KEY_LABEL_WIDTH, rect.top()),
        Pos2::new(rect.right(), rect.top() + HEADER_HEIGHT),
    );
    painter.rect_filled(ruler_rect, 0, BG_ELEVATED);
    painter.rect_stroke(
        ruler_rect,
        0,
        egui::Stroke::new(1.0, BORDER),
        egui::StrokeKind::Inside,
    );

    let beat_duration = 60.0 / 120.0;
    let measure_duration = beat_duration * 4.0;
    let time_start = offset.x / PIXELS_PER_SECOND;
    let time_end = (offset.x + rect.width()) / PIXELS_PER_SECOND;

    let first_measure = (time_start / measure_duration).floor() as i32;
    let last_measure = (time_end / measure_duration).ceil() as i32;

    for m in first_measure..=last_measure {
        let t = m as f32 * measure_duration;
        let x = rect.left() + KEY_LABEL_WIDTH + (t * PIXELS_PER_SECOND) - offset.x;

        if x >= ruler_rect.left() && x <= ruler_rect.right() {
            painter.text(
                Pos2::new(x + 4.0, ruler_rect.center().y),
                egui::Align2::LEFT_CENTER,
                format!("{}.1", m + 1),
                egui::FontId::monospace(10.0),
                TEXT_PRIMARY,
            );

            painter.line_segment(
                [
                    Pos2::new(x, ruler_rect.bottom()),
                    Pos2::new(x, rect.bottom()),
                ],
                egui::Stroke::new(1.0, BORDER_LIGHT),
            );
        }
    }
}

fn draw_header_border(painter: &egui::Painter, rect: Rect) {
    let header_line_y = rect.top() + HEADER_HEIGHT;
    painter.line_segment(
        [
            Pos2::new(rect.left(), header_line_y),
            Pos2::new(rect.right(), header_line_y),
        ],
        egui::Stroke::new(1.0, BORDER),
    );
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
            egui::RichText::new("请先导入 MIDI 文件以查看瀑布流音轨")
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
