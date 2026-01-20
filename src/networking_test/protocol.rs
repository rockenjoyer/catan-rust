use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Reliable = 1,
    Unreliable = 2,
    Chat = 3,
    // => todo: define default channel
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Join,
    TurnOver { player: u8 },
    Something { player: u8 },
    Disconnect { player: u8 },
    GameStart,
    GameEnd,

    Chat { player: u8, message: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Confirmation { player: u8 },
    Join { player: u8 },
    Turn { player: u8 },
    GameStart,
    GameEnd,
    Disconnect { player: u8 },
    Something { player: u8 },
    ServerCrash,
    Ready,

    Chat { player: u8, message: String },
}
