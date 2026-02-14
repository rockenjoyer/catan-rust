use serde::{Serialize, Deserialize};
use crate::backend::game::{DevCard, DevCardInput, Game, GamePhase, Harbor, Player, Tile, TurnPhase, Vertex};
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Reliable = 0, // default (predefined default channel: 0 by bevy_quinnet)
    Unreliable = 1,
    Chat = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Join,
    TurnOver { player: u8 },
    Something { player: u8 },
    Disconnect,
    ClientDisconnect { player: u64 },
    GameStart,
    GameEnd,
    ChatMessage { message: String },

    BuildSettlement { player_id: u8, vertex_id: usize },
    BuildRoad { player_id: u8, vertex1: usize, vertex2: usize },
    BuildCity { player_id: u8, vertex_id: usize },
    BuyDevCard { player_id: u8 },
    PlayDevCard { player_id: u8, card_id: usize, input: DevCardInput },
    RollDice { player_id: u8 },
    MoveRobber { tile_id: usize, victim_id: Option<usize> },
    EndTurn { player_id: u8 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Confirmation { player: u8 },
    Join { player: u8 },
    Turn { player: u8 },
    GameStart,
    GameEnd,
    Disconnect { player: u64 },
    Something { player: u8 },
    ServerCrash,
    Ready,

    ChatMessage { message: String },

    ClientConnected { player: u8 },
    ClientDisconnected { player: u8},

    ActionResult { success: bool, message: String},
}