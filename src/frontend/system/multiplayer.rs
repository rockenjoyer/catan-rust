use bevy::prelude::*;

use crate::frontend::bevy::GameState;

#[derive(Event)]
pub enum MultiplayerAction {
    Host,
    Join { code: String },
    Back,
}

pub fn handle_multiplayer_action(
    action: On<MultiplayerAction>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match &*action {
        MultiplayerAction::Host => {
            next_state.set(GameState::Hosting);
        }
        MultiplayerAction::Join { code } => {
            println!("Joining game with code: {}", code);
            next_state.set(GameState::Joining);
        }
        MultiplayerAction::Back => {
            next_state.set(GameState::MainMenu);
        }
    }
}
