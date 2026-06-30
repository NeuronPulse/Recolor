use eframe::egui;

use crate::models::{Panel, SampleItem, TrackNote};
use crate::theme::{BG_CARD, BG_PAGE, BORDER};
use crate::ui;

pub struct App {
    active_panel: Panel,
    samples: Vec<SampleItem>,
    midi_name: String,
    track_count: usize,
    is_generating: bool,
    generate_progress: f32,
    notes: Vec<TrackNote>,
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

        let notes = vec![
            TrackNote {
                pitch: "C4".into(),
                start: 0.0,
                length: 0.5,
            },
            TrackNote {
                pitch: "E4".into(),
                start: 0.5,
                length: 0.25,
            },
            TrackNote {
                pitch: "G4".into(),
                start: 0.75,
                length: 0.5,
            },
            TrackNote {
                pitch: "A4".into(),
                start: 1.25,
                length: 0.75,
            },
            TrackNote {
                pitch: "G4".into(),
                start: 2.0,
                length: 0.25,
            },
            TrackNote {
                pitch: "E4".into(),
                start: 2.25,
                length: 0.5,
            },
            TrackNote {
                pitch: "C4".into(),
                start: 2.75,
                length: 1.0,
            },
        ];

        Self {
            active_panel: Panel::Library,
            samples,
            midi_name: "Bad_Apple.mid".into(),
            track_count: 3,
            is_generating: false,
            generate_progress: 0.0,
            notes,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    &self.midi_name,
                    self.track_count,
                    &self.samples,
                );
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG_PAGE)
                    .inner_margin(egui::Margin::same(0)),
            )
            .show(ctx, |ui| {
                match self.active_panel {
                    Panel::Library => ui::render_library(ui, &self.samples),
                    Panel::Tracks => ui::render_tracks(ui, &self.notes),
                    Panel::Generate => ui::render_generate(
                        ui,
                        &self.midi_name,
                        self.track_count,
                        &self.samples,
                        &mut self.is_generating,
                        &mut self.generate_progress,
                    ),
                }
            });

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
