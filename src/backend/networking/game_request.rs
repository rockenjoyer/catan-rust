use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClient;

use crate::backend::networking::{protocol::ClientMessage, config::GameMode};

use std::cell::RefCell;
use std::rc::Rc;
use bevy_egui::{EguiContexts, egui};
use std::collections::HashMap;

use crate::backend::game::{Game, GamePhase, RoadBuildingMode, Resource as GameResource, DevCard, DevCardInput};

//resource to track road building state
#[derive(Resource, Default)]
pub struct RoadBuildingState {
    pub last_two_vertices: Vec<usize>,
}

//resource to track building mode in normal play
#[derive(Resource, Default, PartialEq, Clone)]
pub enum BuildingMode {
    #[default]
    None,
    BuildingRoad,
    BuildingSettlement,
    UpgradingCity,
}

//resource to track dev card playing state
#[derive(Resource, Default)]
pub struct DevCardPlayState {
    pub selected_card: Option<(DevCard, usize)>, // (card_type, card_id)
    pub awaiting_input: Option<DevCard>,
    // For Knight
    pub selected_tile: Option<usize>,
    pub selected_victim: Option<usize>,
    // For Monopoly
    pub selected_resource: Option<GameResource>,
    // For Road Building
    pub road_building_roads: Vec<(usize, usize)>,
    // For Year of Plenty
    pub year_resources: Vec<GameResource>,
}

//resource to track robber movement when rolling 7
#[derive(Resource, Default)]
pub struct RobberMoveState {
    pub needs_movement: bool,
    pub selected_tile: Option<usize>,
    pub selected_victim: Option<usize>,
}

pub fn request_build_settlement(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
    vertex_id: usize,
) {
    match mode {
        GameMode::Local => {
            let _ = game.borrow_mut()
                .build_settlement(player_id, vertex_id);
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::BuildSettlement {
                player_id: player_id as u8,
                vertex_id,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}

pub fn request_build_road(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
    v1: usize,
    v2: usize,
) {
    match mode {
        GameMode::Local => {
            let _ = game
                .borrow_mut()
                .build_road(player_id, v1, v2, RoadBuildingMode::Normal);
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::BuildRoad {
                player_id: player_id as u8,
                vertex1: v1,
                vertex2: v2,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}

pub fn request_build_city(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
    vertex_id: usize,
) {
    match mode {
        GameMode::Local => {
            let _ = game.borrow_mut()
                .build_city(player_id, vertex_id);
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::BuildCity {
                player_id: player_id as u8,
                vertex_id,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}

pub fn request_buy_devcard(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
) {
    match mode {
        GameMode::Local => {
            let _ = game.borrow_mut()
                .buy_dev_card(player_id);
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::BuyDevCard {
                player_id: player_id as u8,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}

pub fn request_move_robber(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    tile: usize,
    victim: Option<usize>,
) {
    match mode {
        GameMode::Local => {
            let _ = game.borrow_mut()
                .move_robber(tile, victim);
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::MoveRobber { 
                tile_id: tile, victim_id: victim,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}


pub fn request_play_devcard(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
    card_id: usize,
    input: DevCardInput,
) {
    match mode {
        GameMode::Local => {
            let _ = game.borrow_mut()
                .play_dev_card(player_id, card_id, input);
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::PlayDevCard {
                player_id: player_id as u8,
                card_id,
                input,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}

pub fn request_roll_dice(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
) -> Option<(u8, u8, bool)> {

    match mode {
        GameMode::Local => {
            Some(game.borrow_mut().roll_dice()) // return dice result immediately
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::RollDice {
                player_id: player_id as u8,
            };
            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
            None // multiplayer result will come via network
        }
    }
}

pub fn request_end_turn(
    mode: &GameMode,
    game: &Rc<RefCell<Game>>,
    client: &mut QuinnetClient,
    player_id: usize,
) {
    match mode {
        GameMode::Local => {
            game.borrow_mut().next_turn();
        }
        GameMode::Multiplayer => {
            let msg = ClientMessage::EndTurn {
                player_id: player_id as u8,
            };

            let payload = bincode::serialize(&msg).unwrap();
            client.connection_mut().try_send_payload(payload);
        }
    }
}

