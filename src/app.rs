use egui::{CentralPanel, Context, SidePanel, TopBottomPanel};

use crate::ui::{channels, chat, members, servers};

pub struct DiscordRustApp {
    selected_server: usize,
    selected_channel: String,
    draft: String,
    messages: Vec<chat::Message>,
}

impl Default for DiscordRustApp {
    fn default() -> Self {
        Self {
            selected_server: 0,
            selected_channel: "general".to_owned(),
            draft: String::new(),
            messages: vec![
                chat::Message::new("mira", "welcome in! this is the rust client prototype."),
                chat::Message::new("alex", "the layout now has an Android-first navigation mode."),
                chat::Message::new("you", "next up: connect supported Discord services."),
            ],
        }
    }
}

impl eframe::App for DiscordRustApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mobile = ctx.screen_rect().width() < 760.0;

        TopBottomPanel::top("top_bar").exact_height(48.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(egui::RichText::new("Discord").strong().size(17.0));
                ui.separator();
                if !mobile {
                    ui.label(egui::RichText::new("prototype").color(crate::theme::MUTED));
                } else {
                    ui.label(egui::RichText::new(format!("# {}", self.selected_channel)).color(crate::theme::MUTED));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("●").color(crate::theme::GREEN));
                });
            });
        });

        if mobile {
            TopBottomPanel::top("mobile_servers").exact_height(58.0).show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    for (i, name) in ["D", "R", "G", "M", "+"].iter().enumerate() {
                        let selected = self.selected_server == i;
                        if ui.add_sized([42.0, 38.0], egui::Button::new(
                            egui::RichText::new(*name).strong()
                        ).fill(if selected { crate::theme::ACCENT } else { crate::theme::PANEL })).clicked() {
                            self.selected_server = i;
                        }
                    }
                });
            });

            TopBottomPanel::top("mobile_channels").exact_height(46.0).show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for channel in ["general", "introductions", "media", "questions", "announcements", "off-topic"] {
                        let selected = self.selected_channel == channel;
                        if ui.selectable_label(selected, format!("# {channel}")).clicked() {
                            self.selected_channel = channel.to_owned();
                        }
                    }
                });
            });

            CentralPanel::default().show(ctx, |ui| {
                chat::show(ui, &self.selected_channel, &mut self.messages, &mut self.draft);
            });
        } else {
            SidePanel::left("server_rail").exact_width(68.0).resizable(false).show(ctx, |ui| {
                servers::show(ui, &mut self.selected_server);
            });

            SidePanel::left("channel_panel").exact_width(218.0).resizable(false).show(ctx, |ui| {
                channels::show(ui, self.selected_server, &mut self.selected_channel);
            });

            SidePanel::right("member_panel").exact_width(190.0).resizable(false).show(ctx, |ui| {
                members::show(ui);
            });

            CentralPanel::default().show(ctx, |ui| {
                chat::show(ui, &self.selected_channel, &mut self.messages, &mut self.draft);
            });
        }
    }
}
