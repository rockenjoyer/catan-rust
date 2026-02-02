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
    current_state: Res<State<GameState>>,
) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = egui::vec2(300.0, 600.0);

        egui::Window::new("🔧")
            .frame(egui::Frame::NONE)
            .default_size(default_size)
            .order(egui::Order::Foreground) 
            .movable(false)
            .resizable(false)
            .anchor(egui::Align2::LEFT_BOTTOM, (10.0, -10.0))
            .default_size((300.0, 600.0))
            .default_open(false)
            .show(context, |ui| {
                
                button_style(ui);

                let button_text = if audio_state.is_muted {
                    "🔇 Unmute Music"
                } else {
                    "🔊 Mute Music"
                };
                
                if ui.button(button_text).clicked() {
                    toggle_music(&mut audio_state, &mut audio_instances);
                }
                
                //only show "Return to Main Menu" button when in game
                if current_state.get() == &GameState::InGame {
                    if ui.button("🏠 Return to Main Menu").clicked() {
                        next_state.set(GameState::MainMenu);
                    }
                }
            });
    }
}

fn button_style(ui: &mut egui::Ui) {
    //semi-transparent background for buttons
    let button_color = egui::Color32::from_black_alpha(150);

    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = button_color;
}
