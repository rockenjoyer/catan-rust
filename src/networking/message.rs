use core::fmt;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum GameMessage {
    Turn { player: u8},
    Join { player: u8},
    Disconnect { player: u8},
    Confirmation { player: u8}
    // join, current turn, invalid turn, move?, 
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    // connection error, unexpected disconnect, failed to initiate tcp with client, failed to start server, 
}

impl fmt::Display for GameMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameMessage::Confirmation { player } => write!(f, "Hello player {}!", player),
            GameMessage::Turn { player } => write!(f, "It's player {}'s turn", player),
            GameMessage::Join { player } => write!(f, "Player {} has joined", player),
            GameMessage::Disconnect { player } => write!(f, "Player {} has disconnected.", player),
        }
    }
}