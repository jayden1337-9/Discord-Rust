use egui::{Align, Color32, Layout, RichText, ScrollArea, Ui};

use crate::theme::{ACCENT, INPUT, MUTED, TEXT};

#[derive(Clone)]
pub struct Message {
    pub author: String,
    pub body: String,
}

impl Message {
    pub fn new(author: &str, body: &str) -> Self {
        Self { author: author.to_owned(), body: body.to_owned() }
    }
}

pub fn show(ui: &mut Ui, channel: &str, messages: &mut Vec<Message>, draft: &mut String) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("#").color(MUTED).size(20.0));
        ui.label(RichText::new(channel).strong().size(16.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new("ⓘ").color(MUTED));
            ui.label(RichText::new("⌕").color(MUTED));
        });
    });
    ui.separator();

    ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
        ui.add_space(12.0);
        for message in messages.iter() {
            ui.horizontal_top(|ui| {
                ui.add_sized([38.0, 38.0], egui::Label::new(
                    RichText::new(initials(&message.author)).strong().color(Color32::WHITE)
                        .background_color(ACCENT).size(13.0)
                ));
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&message.author).strong().color(TEXT));
                        ui.label(RichText::new("today").size(10.0).color(MUTED));
                    });
                    ui.label(RichText::new(&message.body).color(TEXT));
                });
            });
            ui.add_space(14.0);
        }
    });

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_sized([ui.available_width() - 58.0, 42.0],
            egui::TextEdit::singleline(draft).hint_text(format!("Message #{channel}")).background_color(INPUT));
        if ui.add_sized([50.0, 42.0], egui::Button::new("➤")).clicked() && !draft.trim().is_empty() {
            messages.push(Message::new("you", draft.trim()));
            draft.clear();
        }
    });
}

fn initials(name: &str) -> String {
    name.chars().next().unwrap_or('?').to_uppercase().to_string()
}
