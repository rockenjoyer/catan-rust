use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::backend::networking::client::ClientState;
use crate::frontend::bevy::GameState;
use crate::frontend::system::multiplayer::{MultiplayerAction, HostState};
use crate::backend::networking::server::ServerPlayers;

pub fn setup_lobby_menu(
    mut contexts: EguiContexts,
    state: Res<ClientState>,
    host_state: Res<HostState>,
    mut commands: Commands,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Lobby");

            ui.separator();

            ui.label(format!("Players connected: {}", state.users.len()));

            for (id, _) in state.users.iter() {
                ui.label(format!("Player {}", id));
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
        });
    });
}
