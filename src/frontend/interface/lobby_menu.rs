use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_quinnet::client::QuinnetClient;

use crate::frontend::interface::main_menu;
use crate::frontend::interface::style::{apply_style, text_with_background};
use crate::backend::networking::client::ClientState;
use crate::backend::networking::config::ConnectionMode;
use crate::frontend::system::chat::{ChatState, render_chat_ui};
use crate::frontend::system::multiplayer::{MultiplayerAction, HostState};
use crate::frontend::visual::startscreen::{StartscreenTexture, draw_background, LogoTexture};

/// Generates a lobby menu after a successfull host attempt
pub fn setup_lobby_menu(
    mut contexts: EguiContexts,
    state: Res<ClientState>,
    host_state: Res<HostState>,
    mut commands: Commands,
    background: Option<Res<StartscreenTexture>>,
    logo_image: Option<Res<LogoTexture>>,
    chat_state: ResMut<ChatState>,
    client: ResMut<QuinnetClient>,
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
            let text_size = (20.0 * scale).clamp(18.0, 28.0);
            let heading_size = (60.0 * scale).clamp(42.0, 52.0);

            main_menu::button_style(ui);

            text_with_background(ui, format!("Lobby"), heading_size);
            
            ui.add_space(10.0);

            // Manual input is required in order for the client to join the host
            // Join code is not needed since the host's local IPv4 adress is needed and can server its purpose
            if host_state.is_host {
                let local_ip: String = ConnectionMode::LAN.to_string();
                let local_ip = local_ip.split(':').next().unwrap().to_string();
                text_with_background(ui, format!("Join code: \n{}", local_ip), text_size);
                
                ui.add_space(10.0);
            }

            text_with_background(ui, format!("Players connected: {}", state.users.len()), text_size);

            for (id, _) in state.users.iter() {
                text_with_background(ui, format!("Player {}", id), text_size);
            }

            ui.add_space(20.0);

            if host_state.is_host {
                if ui
                    .add_enabled(
                        state.users.len() > 1,
                        egui::Button::new("Start Game"),
                    )
                    .clicked()
                {
                    commands.trigger(MultiplayerAction::StartGame);
                }
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
    render_chat_ui(contexts, chat_state, client);
}

