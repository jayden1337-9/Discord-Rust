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
                chat::Message::new("alex", "the layout is split into reusable rust modules."),
                chat::Message::new("you", "next up: polish the mobile layout."),
            ],
        }
    }
}

impl eframe::App for DiscordRustApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_bar").exact_height(48.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(egui::RichText::new("Discord Rust").strong().size(16.0));
                ui.separator();
                ui.label(egui::RichText::new("prototype").color(crate::theme::MUTED));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("● online");
                });
            });
        });

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
