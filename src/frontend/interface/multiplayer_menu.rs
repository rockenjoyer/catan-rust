use bevy::prelude::*;
use bevy_egui::{
    egui::{self, TextEdit},
    EguiContexts,
};

use crate::frontend::interface::style::apply_style;
use crate::frontend::system::multiplayer::MultiplayerAction;

#[derive(Resource, Default)]
pub struct MultiplayerMenuState {
    pub join_code: String,
}

pub fn setup_multiplayer_menu(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut menu_state: ResMut<MultiplayerMenuState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    apply_style(ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {

            let available_size = ui.available_size();
            let base_size = egui::vec2(2048.0, 1152.0);

            let scale = (available_size.x / base_size.x)
                .min(available_size.y / base_size.y)
                .clamp(0.2, 1.0);

            let button_width = (300.0 * scale).clamp(100.0, 340.0);
            let button_height = (80.0 * scale).clamp(25.0, 70.0);
            let button_size = egui::vec2(button_width, button_height);
            let font_size = (20.0 * scale).clamp(12.0, 22.0);

            ui.add_space(200.0 * scale);

            if ui
                .add_sized(
                    button_size,
                    egui::Button::new(
                        egui::RichText::new("Host Game").size(font_size),
                    ),
                )
                .clicked()
            {
                commands.trigger(MultiplayerAction::Host);
            }

            ui.add_space(20.0);

            ui.label(
                egui::RichText::new("Join Game")
                    .size(font_size)
                    .strong(),
            );

            ui.add_space(10.0);

            ui.add_sized(
                egui::vec2(button_width, 30.0 * scale),
                TextEdit::singleline(&mut menu_state.join_code)
                    .hint_text("Enter code"),
            );

            ui.add_space(10.0);

            if ui
                .add_sized(
                    button_size,
                    egui::Button::new(
                        egui::RichText::new("Join").size(font_size),
                    ),
                )
                .clicked()
                && !menu_state.join_code.is_empty()
            {
                commands.trigger(MultiplayerAction::Join {
                    code: menu_state.join_code.clone(),
                });
            }

            ui.add_space(30.0);

            if ui
                .add_sized(
                    button_size,
                    egui::Button::new(
                        egui::RichText::new("Back").size(font_size),
                    ),
                )
                .clicked()
            {
                commands.trigger(MultiplayerAction::Back);
            }
        });
    });
}
