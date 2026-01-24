use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::collections::VecDeque;

//resource to store log messages
#[derive(Resource)]
pub struct GameLog {
    pub messages: VecDeque<LogMessage>,
    pub max_messages: usize,
}

#[derive(Clone)]
pub struct LogMessage {
    pub text: String,
    pub level: LogLevel,
    pub timestamp: f32,
}

#[derive(Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl Default for GameLog {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: 100, //keep last 100 messages
        }
    }
}

impl GameLog {
    pub fn add_info(&mut self, message: String, time: f32) {
        self.add_message(LogMessage {
            text: message,
            level: LogLevel::Info,
            timestamp: time,
        });
    }

    pub fn add_warn(&mut self, message: String, time: f32) {
        self.add_message(LogMessage {
            text: message,
            level: LogLevel::Warn,
            timestamp: time,
        });
    }

    pub fn add_error(&mut self, message: String, time: f32) {
        self.add_message(LogMessage {
            text: message,
            level: LogLevel::Error,
            timestamp: time,
        });
    }

    fn add_message(&mut self, message: LogMessage) {
        self.messages.push_back(message);
        if self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

pub fn setup_log_panel(
    mut contexts: EguiContexts,
    mut game_log: ResMut<GameLog>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Game Log")
            .default_width(300.0)
            .default_height(400.0)
            .resizable(false)
            .collapsible(true)
            .default_open(false)
            .default_pos((10.0, 500.0))
            .movable(true)
            .frame(window_frame())
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("📋 Game Log");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear").clicked() {
                            game_log.clear();
                        }
                    });
                });

                ui.separator();

                //scrollable area for messages
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .max_height(330.0)
                    .show(ui, |ui| {
                        ui.set_width(270.0);
                        for msg in game_log.messages.iter() {
                            let (color, prefix) = match msg.level {
                                LogLevel::Info => (egui::Color32::from_rgb(100, 200, 100), "ℹ️"),
                                LogLevel::Warn => (egui::Color32::from_rgb(255, 200, 0), "⚠️"),
                                LogLevel::Error => (egui::Color32::from_rgb(255, 100, 100), "❌"),
                            };

                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    egui::RichText::new(prefix)
                                        .color(color)
                                );
                                ui.label(
                                    egui::RichText::new(&msg.text)
                                        .color(color)
                                );
                            });

                            ui.add_space(4.0);
                        }
                    });
            });
    }
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_black_alpha(150))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(100)))
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(egui::CornerRadius::same(15))
}