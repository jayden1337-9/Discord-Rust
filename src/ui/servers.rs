use egui::{Button, Ui};

use crate::theme::{ACCENT, MUTED, PANEL, RAIL};

const SERVERS: [&str; 5] = ["D", "R", "G", "M", "+"];

pub fn show(ui: &mut Ui, selected: &mut usize) {
    ui.visuals_mut().panel_fill = RAIL;
    ui.add_space(8.0);
    for (i, name) in SERVERS.iter().enumerate() {
        let fill = if *selected == i { ACCENT } else { PANEL };
        if ui.add_sized([44.0, 44.0], Button::new(egui::RichText::new(*name).size(18.0).strong()).fill(fill)).clicked() {
            *selected = i;
        }
        ui.add_space(8.0);
    }
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.label(egui::RichText::new("●").color(MUTED).size(22.0));
        ui.add_space(12.0);
    });
}
