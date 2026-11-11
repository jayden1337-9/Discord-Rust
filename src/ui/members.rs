use egui::{RichText, Ui};

use crate::theme::MUTED;

pub fn show(ui: &mut Ui) {
    ui.add_space(12.0);
    ui.label(RichText::new("ONLINE — 3").color(MUTED).size(11.0).strong());
    ui.add_space(8.0);
    for name in ["mira", "alex", "you"] {
        ui.horizontal(|ui| {
            ui.label(RichText::new("●").color(crate::theme::GREEN));
            ui.label(name);
        });
        ui.add_space(8.0);
    }
    ui.add_space(16.0);
    ui.label(RichText::new("OFFLINE — 1").color(MUTED).size(11.0).strong());
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("●").color(MUTED));
        ui.label(RichText::new("sam").color(MUTED));
    });
}
