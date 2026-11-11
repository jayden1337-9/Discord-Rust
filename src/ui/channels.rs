use egui::{RichText, Ui};

use crate::theme::{MUTED, TEXT};

const CHANNELS: [&str; 6] = ["general", "introductions", "media", "questions", "announcements", "off-topic"];

pub fn show(ui: &mut Ui, server: usize, selected: &mut String) {
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new(if server == 0 { "my server" } else { "community" }).strong().size(16.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new("⌄").color(MUTED));
        });
    });
    ui.add_space(16.0);
    ui.label(RichText::new("TEXT CHANNELS").color(MUTED).size(11.0).strong());
    ui.add_space(6.0);

    for channel in CHANNELS {
        let active = selected == channel;
        let label = if active { format!("▰  {channel}") } else { format!("#  {channel}") };
        if ui.selectable_label(active, RichText::new(label).color(if active { TEXT } else { MUTED })).clicked() {
            *selected = channel.to_owned();
        }
    }

    ui.add_space(18.0);
    ui.label(RichText::new("VOICE CHANNELS").color(MUTED).size(11.0).strong());
    ui.add_space(6.0);
    ui.label(RichText::new("◖  Lounge").color(MUTED));
}
