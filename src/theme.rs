use egui::{Color32, Context, Visuals};

pub const BG: Color32 = Color32::from_rgb(49, 51, 56);
pub const PANEL: Color32 = Color32::from_rgb(43, 45, 49);
pub const RAIL: Color32 = Color32::from_rgb(30, 31, 34);
pub const INPUT: Color32 = Color32::from_rgb(56, 58, 64);
pub const ACCENT: Color32 = Color32::from_rgb(88, 101, 242);
pub const TEXT: Color32 = Color32::from_rgb(242, 243, 245);
pub const MUTED: Color32 = Color32::from_rgb(148, 155, 164);
pub const GREEN: Color32 = Color32::from_rgb(35, 165, 90);

pub fn configure(ctx: &Context) {
    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(TEXT);
    visuals.panel_fill = BG;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = RAIL;
    visuals.faint_bg_color = INPUT;
    visuals.selection.bg_fill = ACCENT;
    visuals.widgets.noninteractive.bg_fill = PANEL;
    visuals.widgets.inactive.bg_fill = INPUT;
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(70, 73, 80);
    visuals.widgets.active.bg_fill = ACCENT;
    ctx.set_visuals(visuals);
}
