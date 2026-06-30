use eframe::egui;

use crate::audio::AudioEngine;
use crate::core::midi::MidiNote;
use crate::core::project::Project;
use crate::theme::{BG_CARD, BG_PAGE, BORDER};
use crate::ui;
use crate::ui::state::{Panel, ViewState};

pub struct App {
    pub project: Project,
    pub view: ViewState,
    audio: AudioEngine,
}

impl App {
    pub fn new() -> Self {
        let mut project = Project::new();
        project.name = "未命名项目".into();

        Self {
            project,
            view: ViewState::new(),
            audio: AudioEngine::new(),
        }
    }

    fn load_midi_file(&mut self) {
        let dialog = rfd::FileDialog::new()
            .add_filter("MIDI files", &["mid", "midi"])
            .set_title("选择 MIDI 文件");

        if let Some(path) = dialog.pick_file() {
            match self.project.load_midi(&path) {
                Ok(()) => {
                    self.view.error_message = None;
                    self.view.selected_track = None;
                    self.audio.clear_rendered();
                }
                Err(e) => {
                    self.view.error_message = Some(e);
                }
            }
        }
    }

    fn import_samples(&mut self) {
        let dialog = rfd::FileDialog::new()
            .set_title("选择素材目录")
            .pick_folder();

        if let Some(path) = dialog {
            match self.project.import_samples_from_dir(&path) {
                Ok(count) => {
                    self.view.error_message = None;
                    if count == 0 {
                        self.view.error_message = Some("未找到支持的素材文件".into());
                    }
                    for clip in &self.project.samples.clips {
                        if !self.audio.is_sample_loaded(clip.id) {
                            self.audio.load_sample(clip.id, &clip.path);
                        }
                    }
                }
                Err(e) => {
                    self.view.error_message = Some(e);
                }
            }
        }
    }

    fn start_play(&mut self) {
        let notes = self.get_notes_for_display();
        if notes.is_empty() {
            return;
        }
        self.audio.render_midi(&notes);
        self.audio.start_playback();
    }

    fn stop_play(&mut self) {
        self.audio.stop_all();
        self.view.is_playing = false;
        self.view.current_time = 0.0;
    }

    fn get_notes_for_display(&self) -> Vec<MidiNote> {
        if let Some(ref midi) = self.project.midi {
            if let Some(track_idx) = self.view.selected_track {
                midi.notes_for_track(track_idx)
                    .into_iter()
                    .cloned()
                    .collect()
            } else {
                midi.notes.clone()
            }
        } else {
            Vec::new()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Playback tick ──
        if self.view.is_playing {
            let dt = ctx.input(|i| i.predicted_dt);
            self.view.current_time += dt;
            let total = self.project.total_duration();
            if total > 0.0 && self.view.current_time >= total {
                self.view.current_time = 0.0;
                self.view.is_playing = false;
                self.audio.stop_all();
            }
            ctx.request_repaint();
        }

        let notes = self.get_notes_for_display();
        let error_message = self.view.error_message.clone();
        let active_panel = self.view.active_panel.clone();

        let mut stopped = false;
        let mut play_toggled = false;
        let mut new_play_state = self.view.is_playing;

        // ── Top Bar (transport) ──
        ui::render_top_bar(
            ctx,
            ui::TopBarParams {
                active_panel: &mut self.view.active_panel,
                project_name: &self.project.name,
                bpm: self.project.bpm,
                is_playing: &mut new_play_state,
                current_time: &mut self.view.current_time,
                total_duration: self.project.total_duration(),
                track_count: self.project.track_count(),
                sample_count: self.project.samples.len(),
                stopped: &mut stopped,
                play_toggled: &mut play_toggled,
            },
        );

        if stopped {
            self.stop_play();
        } else if play_toggled {
            if new_play_state {
                self.view.is_playing = true;
                self.start_play();
            } else {
                self.view.is_playing = false;
                self.audio.stop_all();
            }
        }

        // ── Sidebar (browser) ──
        let error_for_sidebar = error_message.clone();
        egui::SidePanel::left("browser_panel")
            .resizable(false)
            .exact_width(200.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_CARD)
                    .stroke(egui::Stroke::new(1.0, BORDER)),
            )
            .show(ctx, |ui| {
                ui::render_sidebar(ui, &mut self.view.active_panel, &self.project);

                if let Some(ref error) = error_for_sidebar {
                    ui.add_space(12.0);
                    ui.colored_label(egui::Color32::from_rgb(220, 50, 50), error);
                }
            });

        // ── Main Content ──
        let mut import_midi_clicked = false;
        let mut import_samples_clicked = false;

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG_PAGE)
                    .inner_margin(egui::Margin::same(0)),
            )
            .show(ctx, |ui| match active_panel {
                Panel::Library => {
                    let resp = ui::render_library(ui, &self.project);
                    import_samples_clicked = resp.import_clicked;
                }
                Panel::Tracks => {
                    let resp = ui::render_tracks(
                        ui,
                        &notes,
                        self.project.midi.as_ref(),
                        &self.project.samples,
                        &mut self.view.selected_track,
                        self.view.error_message.as_ref(),
                        &mut self.view.piano_roll,
                        self.view.current_time,
                        self.view.is_playing,
                    );
                    import_midi_clicked = resp.import_clicked;
                }
                Panel::Generate => {
                    ui::render_generate(ui, &self.project, &self.view);
                }
            });

        if import_midi_clicked {
            self.load_midi_file();
        }
        if import_samples_clicked {
            self.import_samples();
        }
    }
}
