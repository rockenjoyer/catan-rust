use bevy::prelude::*;
use crate::frontend::bevy::GameState;

#[derive(Event)]
pub enum NetworkTransition {
    EnterLobby,
    EnterGame,
}

pub fn handle_network_transition(
    flow: On<NetworkTransition>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match &*flow {
        NetworkTransition::EnterLobby => {
            next_state.set(GameState::Lobby);
        }
        NetworkTransition::EnterGame => {
            next_state.set(GameState::InGame);
        }
    }
}