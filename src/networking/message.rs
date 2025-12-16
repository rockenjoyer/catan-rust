use std::fmt;

use serde::{Serialize, Deserialize};


/// Messages to be displayed to all connected clients
/// Shows game based information: whos turn is it?, who did what?, server crash notif, etc.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameMessage {
    Confirmation { player: u8 },
    JoinRequest { player: u8},
    Join { player: u8 },
    GameStart,
    GameEnd,
    Turn { player: u8 },
    TurnOver { player: u8 },
    Something { player: u8 },
    Disconnect { player: u8 },
    ServerCrash,
    // join, current turn, invalid turn, move?, 
}

/// Messages directly to the server
/// Crash reports from clients, connection stats from clients, etc.
/// This or regular logging?
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    JoinRequest { pip: String },
    Joined { player: u8, pip: String },
    Disconnect { player: u8, error: String},
    FailedInit { player: u8, error: String},
    FailedConnect { player: u8, error: String},
    ServerCrash { error: String },
    // connection error, unexpected disconnect, failed to initiate tcp with client, failed to start server, 
    // encrypt public ip or irrelevant?
}