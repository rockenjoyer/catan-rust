use crate::frontend::interface::style::apply_style;
use crate::frontend::system::audio::{AudioState, toggle_music};
use crate::frontend::bevy::GameState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy_kira_audio::AudioInstance;

pub fn setup_settings(
    mut context: EguiContexts,
    mut audio_state: ResMut<AudioState>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = egui::vec2(300.0, 600.0);

        egui::Window::new("Settings")
            .frame(window_frame())
            .default_size(default_size)
            .order(egui::Order::Foreground) 
            .anchor(egui::Align2::LEFT_BOTTOM, (0.0, 0.0))
            .default_size((300.0, 600.0))
            .default_open(false)
            .show(context, |ui| {
                
                let button_text = if audio_state.is_muted {
                    "🔇 Unmute Music"
                } else {
                    "🔊 Mute Music"
                };
                
                if ui.button(button_text).clicked() {
                    toggle_music(&mut audio_state, &mut audio_instances);
                }
                
                if ui.button("🏠 Return to Main Menu").clicked() {
                    next_state.set(GameState::MainMenu);
                }
                
                if ui.button("❌ Quit Game").clicked() {
                    std::process::exit(0);
                }
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
