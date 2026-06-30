use eframe::egui;

use crate::midi::parse_midi_file;
use crate::models::{MidiData, Panel, SampleItem, TrackNote};
use crate::theme::{BG_CARD, BG_PAGE, BORDER};
use crate::ui;
use crate::ui::TrackView;

pub struct App {
    active_panel: Panel,
    samples: Vec<SampleItem>,
    midi_data: Option<MidiData>,
    is_generating: bool,
    generate_progress: f32,
    selected_track: Option<usize>,
    view_mode: TrackView,
    error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let samples = vec![
            SampleItem {
                name: "葛平_天仙子_哼唱.wav".into(),
                duration: "0.24s".into(),
                sample_rate: "44.1kHz".into(),
                pitch: "A4".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "冰红茶_啊.wav".into(),
                duration: "0.18s".into(),
                sample_rate: "44.1kHz".into(),
                pitch: "C#3".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "马里奥_跳跃.wav".into(),
                duration: "0.12s".into(),
                sample_rate: "48kHz".into(),
                pitch: "E5".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "古拉_怒吼.wav".into(),
                duration: "0.56s".into(),
                sample_rate: "44.1kHz".into(),
                pitch: "G2".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "诸葛孔明_出山.wav".into(),
                duration: "0.32s".into(),
                sample_rate: "44.1kHz".into(),
                pitch: "B3".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "鸡你太美_副歌.wav".into(),
                duration: "0.88s".into(),
                sample_rate: "44.1kHz".into(),
                pitch: "D4".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "蓝猫_淘气.wav".into(),
                duration: "0.15s".into(),
                sample_rate: "22.05kHz".into(),
                pitch: "F#4".into(),
                format: "WAV".into(),
            },
            SampleItem {
                name: "奥利奥_干杯.wav".into(),
                duration: "0.42s".into(),
                sample_rate: "44.1kHz".into(),
                pitch: "C5".into(),
                format: "WAV".into(),
            },
        ];

        Self {
            active_panel: Panel::Library,
            samples,
            midi_data: None,
            is_generating: false,
            generate_progress: 0.0,
            selected_track: None,
            view_mode: TrackView::PianoRoll,
            error_message: None,
        }
    }

    fn load_midi_file(&mut self) {
        let dialog = rfd::FileDialog::new()
            .add_filter("MIDI files", &["mid", "midi"])
            .set_title("选择 MIDI 文件");

        if let Some(path) = dialog.pick_file() {
            match parse_midi_file(&path) {
                Ok(midi_data) => {
                    self.midi_data = Some(midi_data);
                    self.error_message = None;
                    self.selected_track = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("MIDI 解析失败: {}", e));
                    self.midi_data = None;
                }
            }
        }
    }

    fn get_notes_for_display(&self) -> Vec<TrackNote> {
        if let Some(ref midi_data) = self.midi_data {
            if let Some(track_idx) = self.selected_track {
                midi_data
                    .notes
                    .iter()
                    .filter(|n| n.track_index == track_idx)
                    .cloned()
                    .collect()
            } else {
                midi_data.notes.clone()
            }
        } else {
            Vec::new()
        }
    }

    fn get_track_count(&self) -> usize {
        self.midi_data
            .as_ref()
            .map(|d| d.tracks.len())
            .unwrap_or(0)
    }

    #[allow(dead_code)]
    fn get_total_notes(&self) -> usize {
        self.midi_data
            .as_ref()
            .map(|d| d.notes.len())
            .unwrap_or(0)
    }

    fn get_midi_name(&self) -> String {
        self.midi_data
            .as_ref()
            .map(|d| d.file_name.clone())
            .unwrap_or_else(|| "未加载".into())
    }

    #[allow(dead_code)]
    fn get_tempo(&self) -> f32 {
        self.midi_data.as_ref().map(|d| d.tempo).unwrap_or(120.0)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let midi_name = self.get_midi_name();
        let track_count = self.get_track_count();
        let error_message = self.error_message.clone();
        let active_panel = self.active_panel.clone();
        let notes = self.get_notes_for_display();

        ui::render_top_bar(ctx, &mut self.active_panel);

        egui::SidePanel::left("sidebar_panel")
            .resizable(false)
            .exact_width(200.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_CARD)
                    .stroke(egui::Stroke::new(1.0, BORDER)),
            )
            .show(ctx, |ui| {
                ui::render_sidebar(
                    ui,
                    &mut self.active_panel,
                    &midi_name,
                    track_count,
                    &self.samples,
                );

                if let Some(ref error) = error_message {
                    ui.add_space(12.0);
                    ui.colored_label(egui::Color32::from_rgb(220, 50, 50), error);
                }
            });

        let mut import_clicked = false;

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG_PAGE)
                    .inner_margin(egui::Margin::same(0)),
            )
            .show(ctx, |ui| {
                match active_panel {
                    Panel::Library => ui::render_library(ui, &self.samples),
                    Panel::Tracks => {
                        let tracks_resp = ui::render_tracks(
                            ui,
                            &notes,
                            self.midi_data.as_ref(),
                            &mut self.selected_track,
                            &mut self.view_mode,
                            self.error_message.as_ref(),
                        );
                        import_clicked = tracks_resp.import_clicked;
                    }
                    Panel::Generate => ui::render_generate(
                        ui,
                        &midi_name,
                        track_count,
                        &self.samples,
                        &mut self.is_generating,
                        &mut self.generate_progress,
                    ),
                }
            });

        if import_clicked {
            self.load_midi_file();
        }

        if self.is_generating {
            self.generate_progress += 0.008;
            if self.generate_progress >= 1.0 {
                self.is_generating = false;
                self.generate_progress = 0.0;
            }
            ctx.request_repaint();
        }
    }
}
