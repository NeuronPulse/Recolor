use egui::Color32;

pub const BG_PAGE: Color32 = Color32::from_rgb(13, 13, 14);
pub const BG_CARD: Color32 = Color32::from_rgb(30, 30, 32);
pub const BG_ELEVATED: Color32 = Color32::from_rgb(38, 38, 41);
pub const BG_HOVER: Color32 = Color32::from_rgb(45, 45, 48);
pub const BORDER: Color32 = Color32::from_rgb(45, 45, 48);
pub const BORDER_LIGHT: Color32 = Color32::from_rgb(55, 55, 58);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(236, 236, 241);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(156, 163, 175);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(110, 110, 128);
pub const ACCENT: Color32 = Color32::from_rgb(16, 163, 127);
pub const MONO: Color32 = Color32::from_rgb(16, 163, 127);

pub fn apply_openai_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    let mut v = egui::Visuals::dark();
    v.panel_fill = BG_PAGE;
    v.window_fill = BG_CARD;
    v.extreme_bg_color = Color32::from_rgb(8, 8, 9);
    v.faint_bg_color = BG_CARD;
    v.widgets.noninteractive.weak_bg_fill = BG_CARD;
    v.widgets.inactive.weak_bg_fill = BG_CARD;
    v.widgets.hovered.weak_bg_fill = BG_HOVER;
    v.widgets.active.weak_bg_fill = BG_ELEVATED;
    v.selection.bg_fill = ACCENT;
    v.selection.stroke = egui::Stroke::new(1.0, Color32::WHITE);
    v.override_text_color = Some(TEXT_PRIMARY);
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, TEXT_SECONDARY);
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT_PRIMARY);
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, TEXT_PRIMARY);
    v.widgets.active.fg_stroke = egui::Stroke::new(1.0, TEXT_PRIMARY);
    v.window_shadow = egui::Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: Color32::from_black_alpha(40),
    };
    style.visuals = v;
    style.spacing.item_spacing = egui::vec2(6.0, 4.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.window_margin = egui::Margin::same(12);

    ctx.set_style(style);
}
