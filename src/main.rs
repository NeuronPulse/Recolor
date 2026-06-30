use eframe::egui;
use std::fs;

fn main() -> eframe::Result {
    let mut fonts = egui::FontDefinitions::default();

    if let Ok(data) = fs::read("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc") {
        fonts.font_data.insert(
            "noto_cjk_regular".to_owned(),
            egui::FontData::from_owned(data).into(),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "noto_cjk_regular".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "noto_cjk_regular".to_owned());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1080.0, 720.0])
            .with_min_inner_size([800.0, 520.0])
            .with_title("Recolor"),
        ..Default::default()
    };

    eframe::run_native(
        "Recolor",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_fonts(fonts);
            apply_openai_style(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}

const BG_PAGE: egui::Color32 = egui::Color32::from_rgb(13, 13, 14);
const BG_CARD: egui::Color32 = egui::Color32::from_rgb(30, 30, 32);
const BG_ELEVATED: egui::Color32 = egui::Color32::from_rgb(38, 38, 41);
const BG_HOVER: egui::Color32 = egui::Color32::from_rgb(45, 45, 48);
const BORDER: egui::Color32 = egui::Color32::from_rgb(45, 45, 48);
const BORDER_LIGHT: egui::Color32 = egui::Color32::from_rgb(55, 55, 58);
const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(236, 236, 241);
const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(156, 163, 175);
const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(110, 110, 128);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(16, 163, 127);
const MONO: egui::Color32 = egui::Color32::from_rgb(16, 163, 127);

fn apply_openai_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    let mut v = egui::Visuals::dark();
    v.panel_fill = BG_PAGE;
    v.window_fill = BG_CARD;
    v.extreme_bg_color = egui::Color32::from_rgb(8, 8, 9);
    v.faint_bg_color = BG_CARD;
    v.widgets.noninteractive.weak_bg_fill = BG_CARD;
    v.widgets.inactive.weak_bg_fill = BG_CARD;
    v.widgets.hovered.weak_bg_fill = BG_HOVER;
    v.widgets.active.weak_bg_fill = BG_ELEVATED;
    v.selection.bg_fill = ACCENT;
    v.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    v.override_text_color = Some(TEXT_PRIMARY);
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, TEXT_SECONDARY);
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT_PRIMARY);
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, TEXT_PRIMARY);
    v.widgets.active.fg_stroke = egui::Stroke::new(1.0, TEXT_PRIMARY);
    v.window_shadow = egui::Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: egui::Color32::from_black_alpha(40),
    };
    style.visuals = v;
    style.spacing.item_spacing = egui::vec2(6.0, 4.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.window_margin = egui::Margin::same(12);

    ctx.set_style(style);
}

#[derive(Clone, PartialEq)]
enum Panel {
    Library,
    Tracks,
    Generate,
}

struct SampleItem {
    name: String,
    duration: String,
    sample_rate: String,
    pitch: String,
    format: String,
}

struct TrackNote {
    pitch: String,
    start: f32,
    length: f32,
}

struct App {
    active_panel: Panel,
    samples: Vec<SampleItem>,
    midi_name: String,
    track_count: usize,
    is_generating: bool,
    generate_progress: f32,
    notes: Vec<TrackNote>,
}

impl App {
    fn new() -> Self {
        let samples = vec![
            SampleItem { name: "葛平_天仙子_哼唱.wav".into(), duration: "0.24s".into(), sample_rate: "44.1kHz".into(), pitch: "A4".into(), format: "WAV".into() },
            SampleItem { name: "冰红茶_啊.wav".into(), duration: "0.18s".into(), sample_rate: "44.1kHz".into(), pitch: "C#3".into(), format: "WAV".into() },
            SampleItem { name: "马里奥_跳跃.wav".into(), duration: "0.12s".into(), sample_rate: "48kHz".into(), pitch: "E5".into(), format: "WAV".into() },
            SampleItem { name: "古拉_怒吼.wav".into(), duration: "0.56s".into(), sample_rate: "44.1kHz".into(), pitch: "G2".into(), format: "WAV".into() },
            SampleItem { name: "诸葛孔明_出山.wav".into(), duration: "0.32s".into(), sample_rate: "44.1kHz".into(), pitch: "B3".into(), format: "WAV".into() },
            SampleItem { name: "鸡你太美_副歌.wav".into(), duration: "0.88s".into(), sample_rate: "44.1kHz".into(), pitch: "D4".into(), format: "WAV".into() },
            SampleItem { name: "蓝猫_淘气.wav".into(), duration: "0.15s".into(), sample_rate: "22.05kHz".into(), pitch: "F#4".into(), format: "WAV".into() },
            SampleItem { name: "奥利奥_干杯.wav".into(), duration: "0.42s".into(), sample_rate: "44.1kHz".into(), pitch: "C5".into(), format: "WAV".into() },
        ];

        let notes = vec![
            TrackNote { pitch: "C4".into(), start: 0.0, length: 0.5 },
            TrackNote { pitch: "E4".into(), start: 0.5, length: 0.25 },
            TrackNote { pitch: "G4".into(), start: 0.75, length: 0.5 },
            TrackNote { pitch: "A4".into(), start: 1.25, length: 0.75 },
            TrackNote { pitch: "G4".into(), start: 2.0, length: 0.25 },
            TrackNote { pitch: "E4".into(), start: 2.25, length: 0.5 },
            TrackNote { pitch: "C4".into(), start: 2.75, length: 1.0 },
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
        self.render_top_bar(ctx);

        egui::SidePanel::left("sidebar_panel")
            .resizable(false)
            .exact_width(200.0)
            .frame(egui::Frame::new().fill(BG_CARD).stroke(egui::Stroke::new(1.0, BORDER)))
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_PAGE).inner_margin(egui::Margin::same(0)))
            .show(ctx, |ui| {
                self.render_main(ui);
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

impl App {
    fn render_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar")
            .frame(
                egui::Frame::new()
                    .fill(BG_CARD)
                    .stroke(egui::Stroke::new(1.0, BORDER))
                    .inner_margin(egui::Margin::symmetric(16, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Re")
                            .size(16.0)
                            .color(ACCENT)
                            .family(egui::FontFamily::Monospace),
                    );
                    ui.label(
                        egui::RichText::new("color")
                            .size(16.0)
                            .color(TEXT_PRIMARY)
                            .family(egui::FontFamily::Monospace),
                    );

                    ui.add_space(24.0);

                    let tabs = [(Panel::Library, "素材库"), (Panel::Tracks, "音轨"), (Panel::Generate, "生成")];
                    for (panel, label) in tabs {
                        let is_active = self.active_panel == panel;
                        let text_color = if is_active { TEXT_PRIMARY } else { TEXT_MUTED };
                        let bg = if is_active { BG_ELEVATED } else { egui::Color32::TRANSPARENT };

                        let btn = egui::Button::new(
                            egui::RichText::new(label).size(13.0).color(text_color),
                        )
                        .min_size(egui::vec2(60.0, 28.0))
                        .corner_radius(4)
                        .fill(bg);
                        if ui.add(btn).clicked() {
                            self.active_panel = panel.clone();
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("v0.1.0")
                                .size(11.0)
                                .color(TEXT_MUTED)
                                .family(egui::FontFamily::Monospace),
                        );
                    });
                });
            });
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);

        ui.label(
            egui::RichText::new("导航")
                .size(11.0)
                .color(TEXT_MUTED)
                .family(egui::FontFamily::Proportional),
        );
        ui.add_space(4.0);

        let nav_items = [
            (Panel::Library, "📚", "素材库"),
            (Panel::Tracks, "🎵", "音轨编辑"),
            (Panel::Generate, "⚡", "一键生成"),
        ];

        for (panel, icon, label) in nav_items {
            let is_active = self.active_panel == panel;
            let bg = if is_active { BG_ELEVATED } else { egui::Color32::TRANSPARENT };
            let text_c = if is_active { TEXT_PRIMARY } else { TEXT_SECONDARY };
            let indicator = if is_active { ACCENT } else { egui::Color32::TRANSPARENT };

            let btn = egui::Button::new(
                egui::RichText::new(format!("{}  {}", icon, label))
                    .size(13.0)
                    .color(text_c),
            )
            .min_size(egui::vec2(ui.available_width(), 32.0))
            .corner_radius(4)
            .fill(bg);

            let response = ui.add(btn);
            if response.clicked() {
                self.active_panel = panel.clone();
            }
            if is_active {
                let rect = response.rect;
                ui.painter().line_segment(
                    [rect.left_top(), rect.left_bottom()],
                    egui::Stroke::new(2.0, indicator),
                );
            }
        }

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(12.0);

        ui.label(
            egui::RichText::new("项目信息")
                .size(11.0)
                .color(TEXT_MUTED),
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("MIDI").size(11.0).color(TEXT_MUTED).family(egui::FontFamily::Monospace));
            ui.label(egui::RichText::new(&self.midi_name).size(11.0).color(TEXT_PRIMARY).family(egui::FontFamily::Monospace));
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("轨数").size(11.0).color(TEXT_MUTED).family(egui::FontFamily::Monospace));
            ui.label(egui::RichText::new(format!("{}", self.track_count)).size(11.0).color(TEXT_PRIMARY).family(egui::FontFamily::Monospace));
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("素材").size(11.0).color(TEXT_MUTED).family(egui::FontFamily::Monospace));
            ui.label(egui::RichText::new(format!("{} 个", self.samples.len())).size(11.0).color(TEXT_PRIMARY).family(egui::FontFamily::Monospace));
        });
    }

    fn render_main(&mut self, ui: &mut egui::Ui) {
        match self.active_panel {
            Panel::Library => self.render_library(ui),
            Panel::Tracks => self.render_tracks(ui),
            Panel::Generate => self.render_generate(ui),
        }
    }

    fn render_library(&self, ui: &mut egui::Ui) {
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new("素材库")
                    .size(16.0)
                    .color(TEXT_PRIMARY)
                    .family(egui::FontFamily::Proportional),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!("{} 个切片", self.samples.len()))
                    .size(11.0)
                    .color(TEXT_MUTED)
                    .family(egui::FontFamily::Monospace),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(12.0);
                let btn = egui::Button::new(
                    egui::RichText::new("+ 导入素材").size(12.0).color(TEXT_PRIMARY),
                )
                .corner_radius(4)
                .fill(BG_ELEVATED)
                .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
                ui.add(btn);
            });
        });

        ui.add_space(8.0);
        ui.add_space(12.0);

        let header_frame = egui::Frame::new()
            .fill(BG_CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(12, 6));

        header_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.add_space(40.0 + 12.0 + 3.0 * 8.0);
                ui.label(egui::RichText::new("文件名").size(11.0).color(TEXT_MUTED));
                ui.add_space(80.0);
                ui.label(egui::RichText::new("时长").size(11.0).color(TEXT_MUTED));
                ui.add_space(40.0);
                ui.label(egui::RichText::new("采样率").size(11.0).color(TEXT_MUTED));
                ui.add_space(40.0);
                ui.label(egui::RichText::new("格式").size(11.0).color(TEXT_MUTED));
                ui.add_space(40.0);
                ui.label(egui::RichText::new("音高").size(11.0).color(TEXT_MUTED));
            });
        });

        ui.add_space(4.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(4.0);
            ui.add_space(12.0);

            for (i, sample) in self.samples.iter().enumerate() {
                self.render_sample_item(ui, sample, i);
                ui.add_space(2.0);
            }
        });
    }

    fn render_sample_item(&self, ui: &mut egui::Ui, sample: &SampleItem, _index: usize) {
        let item_frame = egui::Frame::new()
            .fill(BG_CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(4)
            .inner_margin(egui::Margin::symmetric(12, 8));

        item_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.horizontal(|ui| {
                let format_bg = egui::Color32::from_rgb(20, 20, 22);
                let format_frame = egui::Frame::new()
                    .fill(format_bg)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::same(6))
                    .stroke(egui::Stroke::new(1.0, BORDER));

                format_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&sample.format)
                            .size(10.0)
                            .color(TEXT_MUTED)
                            .family(egui::FontFamily::Monospace),
                    );
                });

                ui.add_space(4.0);

                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&sample.name)
                            .size(13.0)
                            .color(TEXT_PRIMARY),
                    );
                    ui.add_space(2.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("时长: {}", sample.duration))
                                .size(11.0)
                                .color(TEXT_MUTED),
                        );
                        ui.label(egui::RichText::new("·").size(11.0).color(TEXT_MUTED));
                        ui.label(
                            egui::RichText::new(format!("采样率: {}", sample.sample_rate))
                                .size(11.0)
                                .color(TEXT_MUTED),
                        );
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let pitch_frame = egui::Frame::new()
                        .fill(BG_ELEVATED)
                        .corner_radius(4)
                        .inner_margin(egui::Margin::symmetric(8, 3))
                        .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));

                    pitch_frame.show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&sample.pitch)
                                .size(12.0)
                                .color(MONO)
                                .family(egui::FontFamily::Monospace),
                        );
                    });
                });
            });
        });
    }

    fn render_tracks(&self, ui: &mut egui::Ui) {
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
                egui::RichText::new(format!("{} 个音符", self.notes.len()))
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

            for (i, note) in self.notes.iter().enumerate() {
                self.render_note_row(ui, note, i);
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

    fn render_note_row(&self, ui: &mut egui::Ui, note: &TrackNote, index: usize) {
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
                                .color(MONO)
                                .family(egui::FontFamily::Monospace),
                        );
                    });

                    ui.add_space(12.0);

                    let note_label = format!("{:.2}s - {:.2}s", note.start, note.start + note.length);
                    ui.label(
                        egui::RichText::new(&note_label)
                            .size(10.0)
                            .color(TEXT_MUTED)
                            .family(egui::FontFamily::Monospace),
                    );
                });
        });
    }

    fn render_generate(&mut self, ui: &mut egui::Ui) {
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new("一键生成")
                    .size(16.0)
                    .color(TEXT_PRIMARY),
            );
        });

        ui.add_space(16.0);
        ui.add_space(12.0);

        let config_frame = egui::Frame::new()
            .fill(BG_CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(6)
            .inner_margin(egui::Margin::same(16));

        config_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.label(
                egui::RichText::new("生成配置")
                    .size(13.0)
                    .color(TEXT_SECONDARY),
            );
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("MIDI").size(12.0).color(TEXT_SECONDARY).family(egui::FontFamily::Monospace));
                ui.add_space(8.0);
                let midi_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
                midi_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&self.midi_name)
                            .size(12.0)
                            .color(TEXT_PRIMARY)
                            .family(egui::FontFamily::Monospace),
                    );
                });
                ui.label(egui::RichText::new(format!("({} Tracks)", self.track_count)).size(11.0).color(TEXT_MUTED));
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("素材").size(12.0).color(TEXT_SECONDARY).family(egui::FontFamily::Monospace));
                ui.add_space(8.0);
                let sample_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
                sample_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("{} 个切片已就绪", self.samples.len()))
                            .size(12.0)
                            .color(TEXT_PRIMARY)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("算法").size(12.0).color(TEXT_SECONDARY).family(egui::FontFamily::Monospace));
                ui.add_space(8.0);
                let algo_frame = egui::Frame::new()
                    .fill(BG_ELEVATED)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .stroke(egui::Stroke::new(1.0, BORDER_LIGHT));
                algo_frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Pitch-Match v2")
                            .size(12.0)
                            .color(TEXT_PRIMARY)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            });
        });

        ui.add_space(16.0);
        ui.add_space(12.0);

        let result_frame = egui::Frame::new()
            .fill(BG_CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(6)
            .inner_margin(egui::Margin::same(16));

        result_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.label(
                egui::RichText::new("输出预览")
                    .size(13.0)
                    .color(TEXT_SECONDARY),
            );
            ui.add_space(12.0);

            if self.is_generating {
                ui.add_space(8.0);

                let progress_bg = egui::Color32::from_rgb(20, 20, 22);
                let progress_frame = egui::Frame::new()
                    .fill(progress_bg)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::same(0));

                progress_frame.show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    let available_w = ui.available_width();
                    let (r, _) = ui.allocate_exact_size(
                        egui::vec2(available_w, 6.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(r, 3, BORDER);
                    let fill_rect = egui::Rect::from_min_size(
                        r.min,
                        egui::vec2(r.width() * self.generate_progress, r.height()),
                    );
                    ui.painter().rect_filled(fill_rect, 3, ACCENT);
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("正在编排...")
                            .size(12.0)
                            .color(TEXT_SECONDARY),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(format!("{}%", (self.generate_progress * 100.0) as i32))
                            .size(12.0)
                            .color(TEXT_MUTED)
                            .family(egui::FontFamily::Monospace),
                    );
                });
            } else {
                ui.label(
                    egui::RichText::new("等待生成...")
                        .size(12.0)
                        .color(TEXT_MUTED),
                );
            }
        });

        ui.add_space(16.0);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.add_space(12.0);

            let generate_btn = egui::Button::new(
                egui::RichText::new("⚡  一键编排并演奏")
                    .size(13.0)
                    .color(egui::Color32::WHITE)
                    .family(egui::FontFamily::Proportional),
            )
            .corner_radius(4)
            .fill(if self.is_generating { BG_HOVER } else { ACCENT })
            .min_size(egui::vec2(180.0, 36.0));

            if ui.add_enabled(!self.is_generating, generate_btn).clicked() {
                self.is_generating = true;
                self.generate_progress = 0.0;
            }

            ui.add_space(8.0);

            let export_btn = egui::Button::new(
                egui::RichText::new("导出音频")
                    .size(13.0)
                    .color(TEXT_SECONDARY),
            )
            .corner_radius(4)
            .fill(BG_ELEVATED)
            .stroke(egui::Stroke::new(1.0, BORDER_LIGHT))
            .min_size(egui::vec2(100.0, 36.0));
            ui.add_enabled(false, export_btn);
        });
    }
}