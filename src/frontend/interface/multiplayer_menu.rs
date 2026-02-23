use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::frontend::interface::main_menu;
use crate::frontend::interface::style::{apply_style, text_with_background};
use crate::frontend::system::multiplayer::MultiplayerAction;
use crate::frontend::visual::startscreen::{StartscreenTexture, draw_background, LogoTexture};

#[derive(Resource, Default)]
pub struct MultiplayerMenuState {
    pub host_ip: String,
    pub join_code: String,
}

pub fn setup_multiplayer_menu(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut menu_state: ResMut<MultiplayerMenuState>,
    background: Option<Res<StartscreenTexture>>,
    logo_image: Option<Res<LogoTexture>>,
) {
    let Some(background) = background else {
        return;
    };
    let Some(logo_image) = logo_image else {
        return;
    };

    let Ok(ctx) = contexts.ctx_mut() else { return; };
    apply_style(ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        draw_background(ui, &background, &logo_image, ui.available_size());
        ui.vertical_centered(|ui| {
            let available_size = ui.available_size();
            let base_size = egui::vec2(2048.0, 1152.0);
            let scale = (available_size.x / base_size.x)
                .min(available_size.y / base_size.y)
                .clamp(0.2, 1.0);

            let top_space = (450.0 * scale).min(420.0);
            ui.add_space(top_space);

            let button_width = (300.0 * scale).clamp(100.0, 340.0);
            let button_height = (80.0 * scale).clamp(25.0, 70.0);
            let button_size = egui::vec2(button_width, button_height);
            let font_size = (20.0 * scale).clamp(12.0, 22.0);

            main_menu::button_style(ui);

            if ui
                .add_sized(button_size, egui::Button::new(egui::RichText::new("Host Game").size(font_size)))
                .clicked()
            {
                commands.trigger(MultiplayerAction::Host);
            }

            ui.add_space(15.0);

            text_with_background(ui, format!("Join Game"), font_size);

            ui.add_space(10.0);

            // The actual join code is set to an empty string
            // This field is used to input the local IP of the host
            // Instead of the rng join code, the client only needs to input the host local ip in order to join the lobby
            ui.add_sized(
                egui::vec2(button_width, 30.0 * scale),
                egui::TextEdit::singleline(&mut menu_state.host_ip)
                    .hint_text("Join code"),
            );
            ui.add_space(15.0);

            /*
            ui.add_sized(
                egui::vec2(button_width, 30.0 * scale),
                egui::TextEdit::singleline(&mut menu_state.join_code)
                    .hint_text("Join code"),
            );
            */
            ui.add_space(15.0);

            if ui
                .add_sized(button_size, egui::Button::new(egui::RichText::new("Join").size(font_size)))
                .clicked()
                && !menu_state.join_code.is_empty()
            {
                commands.trigger(MultiplayerAction::Join {
                    host_ip: format!("{}:4000", menu_state.host_ip.clone()),
                    code: menu_state.join_code.clone(),
                });
            }

            ui.add_space(30.0);
            if ui
                .add_sized(button_size, egui::Button::new(egui::RichText::new("Back").size(font_size)))
                .clicked()
            {
                commands.trigger(MultiplayerAction::Back);
            }
        });
    });
}

