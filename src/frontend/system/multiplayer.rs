use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClient;

use crate::frontend::bevy::GameState;
use crate::backend::networking::protocol::ClientMessage;
use crate::backend::networking::client::PendingJoin;

#[derive(Event)]
pub enum MultiplayerAction {
    Host,
    Join { host_ip: String, code: String },
    Back,
    EnterLobby,
    StartGame,
}

#[derive(Resource, Default, PartialEq)]
pub struct HostState {
    pub is_host: bool,
}

#[derive(Resource, Default)]
pub struct GameStartOrigin {
    pub started_from_lobby: bool,
}

pub fn handle_multiplayer_action(
    action: On<MultiplayerAction>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut host_state: ResMut<HostState>,
    mut client: ResMut<QuinnetClient>,
) {
    match &*action {
        MultiplayerAction::Host => {
            println!("Hosting game...");
            host_state.is_host = true;

            next_state.set(GameState::Hosting);
        }
        
        MultiplayerAction::Join { host_ip, code } => {
            println!("Joining {} with code: {}", host_ip, code);

            commands.insert_resource(PendingJoin {
                join_code: code.clone(),
            });

            next_state.set(GameState::Joining);
        }

        MultiplayerAction::Back => {
            next_state.set(GameState::MainMenu);
        }

        MultiplayerAction::EnterLobby => {
            next_state.set(GameState::Lobby);
        }

        MultiplayerAction::StartGame => {
            if host_state.is_host {
                println!("Host attempting to start game");

                if let Some(connection) = client.get_connection_mut() {
                    let msg = ClientMessage::GameStart;
                    let payload = bincode::serialize(&msg).unwrap();
                    let _ = connection.try_send_payload(payload);
                }
            }
            next_state.set(GameState::MultiplayerInGame);
        }
    }
}
