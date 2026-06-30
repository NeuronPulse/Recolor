use egui::{Color32, Pos2, Rect, Vec2};

use crate::core::midi::MidiNote;
use crate::theme::{self, BG_CARD, BG_ELEVATED, BORDER, BORDER_LIGHT, TEXT_MUTED, TEXT_PRIMARY};

pub const DEFAULT_ROW_HEIGHT: f32 = 18.0;
pub const MIN_ROW_HEIGHT: f32 = 8.0;
pub const MAX_ROW_HEIGHT: f32 = 40.0;
const NOTE_HEIGHT_RATIO: f32 = 0.78;
pub const DEFAULT_PIXELS_PER_SECOND: f32 = 120.0;
pub const MIN_PPS: f32 = 20.0;
pub const MAX_PPS: f32 = 800.0;
const KEY_LABEL_WIDTH: f32 = 48.0;
const HEADER_HEIGHT: f32 = 28.0;
const PADDING_KEYS: u8 = 3;

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

pub struct PianoRollState {
    pub row_height: f32,
    pub pixels_per_second: f32,
}

impl Default for PianoRollState {
    fn default() -> Self {
        Self {
            row_height: DEFAULT_ROW_HEIGHT,
            pixels_per_second: DEFAULT_PIXELS_PER_SECOND,
        }
    }
}

pub fn compute_key_range(notes: &[MidiNote]) -> (u8, u8) {
    if notes.is_empty() {
        return (36, 84);
    }
    let min_key = notes.iter().map(|n| n.key).min().unwrap_or(36);
    let max_key = notes.iter().map(|n| n.key).max().unwrap_or(84);
    let lo = min_key.saturating_sub(PADDING_KEYS);
    let hi = (max_key + PADDING_KEYS).min(127);
    let range = hi.saturating_sub(lo);
    if range < 24 {
        let mid = (lo + hi) / 2;
        let half = 12;
        let lo = mid.saturating_sub(half);
        let hi = (mid + half).min(127);
        (lo, hi)
    } else {
        (lo, hi)
    }
}

pub struct PianoRollResponse {
    pub import_clicked: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn render_piano_roll(
    ui: &mut egui::Ui,
    notes: &[MidiNote],
    has_midi: bool,
    error_message: Option<&String>,
    state: &mut PianoRollState,
    current_time: f32,
    is_playing: bool,
    bpm: f32,
) -> PianoRollResponse {
    let mut response = PianoRollResponse {
        import_clicked: false,
    };

    if !has_midi {
        response.import_clicked = render_empty_state(ui, error_message);
        return response;
    }

    let (min_key, max_key) = compute_key_range(notes);
    let key_count = (max_key - min_key + 1) as f32;
    let row_height = state.row_height;
    let pps = state.pixels_per_second;
    let note_height = (row_height * NOTE_HEIGHT_RATIO).max(4.0);
    let note_y_pad = (row_height - note_height) / 2.0;

    let total_height = key_count * row_height + HEADER_HEIGHT;
    let available = ui.available_size();
    let height = total_height.min(available.y - 8.0).max(200.0);

    let total_time = if !notes.is_empty() {
        notes
            .iter()
            .map(|n| n.start + n.length)
            .fold(0.0f32, f32::max)
    } else {
        30.0
    };
    let total_time_width = total_time * pps + 200.0;
    let content_width = available.x.max(total_time_width);
    let total_width = content_width + KEY_LABEL_WIDTH;

    let total_size = Vec2::new(total_width, total_height);

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
                let (response, painter) = ui.allocate_painter(total_size, egui::Sense::hover());
                let rect = response.rect;

                draw_background(&painter, rect);
                draw_grid(&painter, rect, pps, row_height, min_key, max_key, bpm);
                draw_notes(
                    &painter,
                    rect,
                    pps,
                    row_height,
                    note_height,
                    note_y_pad,
                    min_key,
                    max_key,
                    notes,
                );
                draw_piano_keys(&painter, rect, row_height, min_key, max_key);
                draw_time_ruler(&painter, rect, pps, bpm);
                draw_header_border(&painter, rect);
                draw_playhead(&painter, rect, pps, current_time);

                // auto-scroll: keep playhead centered in viewport
                if is_playing && current_time > 0.0 {
                    let playhead_x = KEY_LABEL_WIDTH + current_time * pps;
                    let playhead_rect = egui::Rect::from_min_size(
                        egui::Pos2::new(playhead_x, rect.top()),
                        egui::vec2(2.0, rect.height()),
                    );
                    ui.scroll_to_rect(playhead_rect, Some(egui::Align::Center));
                }
            });
    });

    response
}

fn draw_background(painter: &egui::Painter, rect: Rect) {
    painter.rect_filled(rect, 0, BG_CARD);
}

fn draw_grid(
    painter: &egui::Painter,
    rect: Rect,
    pps: f32,
    row_height: f32,
    min_key: u8,
    max_key: u8,
    bpm: f32,
) {
    for key in min_key..=max_key {
        let y = (key - min_key) as f32 * row_height + HEADER_HEIGHT;
        if y + row_height < rect.top() || y > rect.bottom() {
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
            Pos2::new(rect.right(), rect.top() + y + row_height),
        );
        painter.rect_filled(row_rect, 0, row_color);

        painter.line_segment(
            [row_rect.left_top(), row_rect.right_top()],
            egui::Stroke::new(0.5, BORDER),
        );
    }

    let beat_duration = 60.0 / bpm.max(1.0);
    let measure_duration = beat_duration * 4.0;

    let first_measure = ((rect.left() / pps) / measure_duration).floor() as i32 - 1;
    let last_measure = ((rect.right() / pps) / measure_duration).ceil() as i32 + 1;

    for m in first_measure.max(0)..=last_measure {
        let t = m as f32 * measure_duration;
        let x = rect.left() + KEY_LABEL_WIDTH + (t * pps);
        if x >= rect.left() && x <= rect.right() {
            painter.line_segment(
                [
                    Pos2::new(x, rect.top() + HEADER_HEIGHT),
                    Pos2::new(x, rect.bottom()),
                ],
                egui::Stroke::new(1.0, BORDER_LIGHT),
            );
        }

        for beat in 1..4 {
            let bt = t + beat as f32 * beat_duration;
            let bx = rect.left() + KEY_LABEL_WIDTH + (bt * pps);
            if bx >= rect.left() && bx <= rect.right() {
                painter.line_segment(
                    [
                        Pos2::new(bx, rect.top() + HEADER_HEIGHT),
                        Pos2::new(bx, rect.bottom()),
                    ],
                    egui::Stroke::new(0.5, BORDER),
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_notes(
    painter: &egui::Painter,
    rect: Rect,
    pps: f32,
    row_height: f32,
    note_height: f32,
    note_y_pad: f32,
    min_key: u8,
    max_key: u8,
    notes: &[MidiNote],
) {
    for note in notes {
        if note.key < min_key || note.key > max_key {
            continue;
        }

        let note_y = (note.key - min_key) as f32 * row_height + HEADER_HEIGHT + note_y_pad;
        if note_y + note_height < rect.top() || note_y > rect.bottom() {
            continue;
        }

        let x = rect.left() + KEY_LABEL_WIDTH + (note.start * pps);
        let w = (note.length * pps).max(3.0);

        let note_rect =
            Rect::from_min_size(Pos2::new(x, rect.top() + note_y), Vec2::new(w, note_height));

        let base_color = TRACK_COLORS[note.track_index % TRACK_COLORS.len()];
        let alpha = (note.velocity as f32 / 127.0 * 180.0 + 75.0) as u8;
        let note_color =
            Color32::from_rgba_premultiplied(base_color.r(), base_color.g(), base_color.b(), alpha);

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

fn draw_piano_keys(painter: &egui::Painter, rect: Rect, row_height: f32, min_key: u8, max_key: u8) {
    let key_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + HEADER_HEIGHT),
        Pos2::new(rect.left() + KEY_LABEL_WIDTH, rect.bottom()),
    );
    painter.rect_filled(key_rect, 0, BG_ELEVATED);
    painter.rect_stroke(
        key_rect,
        0,
        egui::Stroke::new(1.0, BORDER),
        egui::StrokeKind::Inside,
    );

    for key in min_key..=max_key {
        let y = (key - min_key) as f32 * row_height + HEADER_HEIGHT;
        if y + row_height < rect.top() || y > rect.bottom() {
            continue;
        }

        let note_idx = (key % 12) as usize;
        let is_black = matches!(note_idx, 1 | 3 | 6 | 8 | 10);
        let octave = (key / 12) as i8 - 1;

        let row_rect = Rect::from_min_max(
            Pos2::new(rect.left(), rect.top() + y),
            Pos2::new(rect.left() + KEY_LABEL_WIDTH, rect.top() + y + row_height),
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

fn draw_time_ruler(painter: &egui::Painter, rect: Rect, pps: f32, bpm: f32) {
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

    let beat_duration = 60.0 / bpm.max(1.0);
    let measure_duration = beat_duration * 4.0;

    let first_measure = ((rect.left() / pps) / measure_duration).floor() as i32 - 1;
    let last_measure = ((rect.right() / pps) / measure_duration).ceil() as i32 + 1;

    for m in first_measure.max(0)..=last_measure {
        let t = m as f32 * measure_duration;
        let x = rect.left() + KEY_LABEL_WIDTH + (t * pps);

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

fn draw_playhead(painter: &egui::Painter, rect: Rect, pps: f32, current_time: f32) {
    let x = rect.left() + KEY_LABEL_WIDTH + (current_time * pps);
    if x < rect.left() || x > rect.right() {
        return;
    }

    let top = rect.top();
    let bottom = rect.bottom();

    // playhead line
    painter.line_segment(
        [Pos2::new(x, top), Pos2::new(x, bottom)],
        egui::Stroke::new(2.0, theme::ACCENT),
    );

    // triangle marker at top
    let marker_size = 6.0;
    let marker = [
        Pos2::new(x, top),
        Pos2::new(x - marker_size, top + marker_size),
        Pos2::new(x + marker_size, top + marker_size),
    ];
    painter.add(egui::Shape::convex_polygon(
        marker.to_vec(),
        theme::ACCENT,
        egui::Stroke::NONE,
    ));
}

fn render_empty_state(ui: &mut egui::Ui, error_message: Option<&String>) -> bool {
    let mut import_clicked = false;

    ui.add_space(40.0);

    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("🎵").size(48.0).color(TEXT_MUTED));
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
