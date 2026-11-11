use egui::{Align, CentralPanel, Context, Layout, SidePanel, TopBottomPanel};

use crate::ui::{chat, members, servers};

const CHANNELS: [&str; 6] = [
    "general",
    "introductions",
    "media",
    "questions",
    "announcements",
    "off-topic",
];

pub struct DiscordRustApp {
    selected_server: usize,
    selected_channel: String,
    draft: String,
    messages: Vec<chat::Message>,
    show_channels: bool,
    selected_tab: usize,
}

impl Default for DiscordRustApp {
    fn default() -> Self {
        Self {
            selected_server: 0,
            selected_channel: "general".to_owned(),
            draft: String::new(),
            messages: vec![
                chat::Message::new("mira", "welcome in! this is the new rust client shell."),
                chat::Message::new("alex", "the mobile layout is being rebuilt around the chat view."),
                chat::Message::new("you", "messages here are local preview data for now."),
            ],
            show_channels: false,
            selected_tab: 0,
        }
    }
}

impl eframe::App for DiscordRustApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mobile = ctx.screen_rect().width() < 760.0;

        TopBottomPanel::top("app_header")
            .exact_height(54.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if mobile {
                        if ui.button("☰").clicked() {
                            self.show_channels = !self.show_channels;
                        }
                    }
                    ui.label(egui::RichText::new("Discord").strong().size(18.0));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(if mobile {
                            format!("# {}", self.selected_channel)
                        } else {
                            "Rust client preview".to_owned()
                        })
                        .color(crate::theme::MUTED)
                        .size(13.0),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(egui::RichText::new("●").color(crate::theme::GREEN));
                        ui.label(egui::RichText::new("online").color(crate::theme::MUTED).size(12.0));
                    });
                });
            });

        if mobile {
            TopBottomPanel::top("mobile_server_strip")
                .exact_height(58.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            for (index, name) in ["Home", "D", "R", "G", "M", "+"].iter().enumerate() {
                                let selected = self.selected_server == index;
                                let button = egui::Button::new(
                                    egui::RichText::new(*name).strong().size(15.0),
                                )
                                .fill(if selected {
                                    crate::theme::ACCENT
                                } else {
                                    crate::theme::PANEL
                                });
                                if ui.add_sized([42.0, 42.0], button).clicked() {
                                    self.selected_server = index;
                                }
                                ui.add_space(5.0);
                            }
                        });
                    });
                });

            if self.show_channels {
                SidePanel::left("mobile_channel_drawer")
                    .exact_width(248.0)
                    .resizable(false)
                    .show(ctx, |ui| {
                        channel_list(ui, self.selected_server, &mut self.selected_channel);
                    });
            }

            CentralPanel::default().show(ctx, |ui| {
                chat::show(ui, &self.selected_channel, &mut self.messages, &mut self.draft);
            });

            TopBottomPanel::bottom("mobile_bottom_nav")
                .exact_height(54.0)
                .show(ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        for (index, label) in ["Messages", "Notifications", "You"].iter().enumerate() {
                            let selected = self.selected_tab == index;
                            if ui.selectable_label(selected, *label).clicked() {
                                self.selected_tab = index;
                            }
                            ui.add_space(18.0);
                        }
                    });
                });
        } else {
            SidePanel::left("server_rail")
                .exact_width(68.0)
                .resizable(false)
                .show(ctx, |ui| servers::show(ui, &mut self.selected_server));

            SidePanel::left("channel_panel")
                .exact_width(224.0)
                .resizable(false)
                .show(ctx, |ui| channel_list(ui, self.selected_server, &mut self.selected_channel));

            SidePanel::right("member_panel")
                .exact_width(190.0)
                .resizable(false)
                .show(ctx, |ui| members::show(ui));

            CentralPanel::default().show(ctx, |ui| {
                chat::show(ui, &self.selected_channel, &mut self.messages, &mut self.draft);
            });
        }

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

fn channel_list(ui: &mut egui::Ui, server: usize, selected: &mut String) {
    ui.add_space(12.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(if server == 0 { "Direct Messages" } else { "Community" })
                .strong()
                .size(16.0),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(egui::RichText::new("⌄").color(crate::theme::MUTED));
        });
    });
    ui.separator();
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new("TEXT CHANNELS")
            .color(crate::theme::MUTED)
            .size(11.0)
            .strong(),
    );
    ui.add_space(6.0);

    for channel in CHANNELS {
        let active = selected == channel;
        let label = if active {
            format!("▰  {channel}")
        } else {
            format!("#  {channel}")
        };
        if ui
            .selectable_label(
                active,
                egui::RichText::new(label).color(if active {
                    crate::theme::TEXT
                } else {
                    crate::theme::MUTED
                }),
            )
            .clicked()
        {
            *selected = channel.to_owned();
        }
    }

    ui.add_space(18.0);
    ui.label(
        egui::RichText::new("VOICE CHANNELS")
            .color(crate::theme::MUTED)
            .size(11.0)
            .strong(),
    );
    ui.add_space(6.0);
    ui.label(egui::RichText::new("◖  Lounge").color(crate::theme::MUTED));
}
