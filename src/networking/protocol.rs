use serde::{Serialize, Deserialize};

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
}
