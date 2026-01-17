use serde::{Deserialize, Serialize};

// client -> server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join,
    Disconnect { player: u8 },

    GameStart,
    GameEnd,
    
    TurnOver { player: u8 },
    Something { player: u8 },
}

// server -> clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Confirmation { player: u8 },
    Join { player: u8 },

    GameStart,
    GameEnd,

    Turn { player: u8 },
    TurnOver { player: u8 },

    Something { player: u8 },
    Disconnect { player: u8 },

    ServerCrash,
}
