use egui::{Align, Color32, Layout, RichText, ScrollArea, Ui};

use crate::theme::{ACCENT, INPUT, MUTED, TEXT};

#[derive(Clone)]
pub struct Message {
    pub author: String,
    pub body: String,
}

impl Message {
    pub fn new(author: &str, body: &str) -> Self {
        Self {
            author: author.to_owned(),
            body: body.to_owned(),
        }
    }
}

pub fn show(ui: &mut Ui, channel: &str, messages: &mut Vec<Message>, draft: &mut String) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("#").color(MUTED).size(21.0));
        ui.label(RichText::new(channel).strong().size(16.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new("ⓘ").color(MUTED));
            ui.label(RichText::new("⌕").color(MUTED));
        });
    });
    ui.separator();

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.add_space(14.0);
            for message in messages.iter() {
                ui.horizontal_top(|ui| {
                    let avatar = RichText::new(initials(&message.author))
                        .strong()
                        .color(Color32::WHITE)
                        .background_color(avatar_color(&message.author))
                        .size(14.0);
                    ui.add_sized([38.0, 38.0], egui::Label::new(avatar));

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&message.author).strong().color(TEXT));
                            ui.label(RichText::new("today").size(10.0).color(MUTED));
                        });
                        ui.add_space(2.0);
                        ui.label(RichText::new(&message.body).color(TEXT));
                    });
                });
                ui.add_space(13.0);
            }
        });

    ui.add_space(7.0);
    ui.horizontal(|ui| {
        if ui
            .add_sized([40.0, 43.0], egui::Button::new(RichText::new("+").size(20.0)))
            .clicked()
        {
            // Attachments will be wired when the supported API layer is added.
        }

        let input_width = (ui.available_width() - 54.0).max(80.0);
        let response = ui.add_sized(
            [input_width, 43.0],
            egui::TextEdit::singleline(draft)
                .hint_text(format!("Message #{channel}"))
                .background_color(INPUT),
        );
        let send = ui
            .add_sized([48.0, 43.0], egui::Button::new("➤"))
            .clicked();

        if (send || (response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter))))
            && !draft.trim().is_empty()
        {
            messages.push(Message::new("you", draft.trim()));
            draft.clear();
        }
    });
}

fn initials(name: &str) -> String {
    name.chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string()
}

fn avatar_color(name: &str) -> Color32 {
    let palette = [
        Color32::from_rgb(88, 101, 242),
        Color32::from_rgb(59, 165, 92),
        Color32::from_rgb(235, 69, 158),
        Color32::from_rgb(230, 126, 34),
    ];
    let index = name.bytes().fold(0usize, |value, byte| value.wrapping_add(byte as usize));
    palette[index % palette.len()]
}
