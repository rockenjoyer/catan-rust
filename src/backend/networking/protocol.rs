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
    RollDice { player_id: u8, die1: u8, die2: u8 },
    MoveRobber { player_id: u8, tile_id: usize, victim_id: Option<usize> },
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

    GameStateUpdate { game: GameDTO },
    ActionResult { success: bool, message: String},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDTO {
    pub players: Vec<Player>,
    pub vertices: Vec<Vertex>,
    pub tiles: Vec<Tile>,
    pub harbors: Vec<Harbor>,
    pub robber_tile: usize,
    pub current_player: usize,
    pub dev_card_pool: Vec<DevCard>,
    pub turn_phase: TurnPhase,
    pub game_phase: GamePhase,
    pub setup_placement: u8,
    pub largest_army_owner: Option<usize>,
    pub longest_road_owner: Option<usize>,
    pub longest_road_length: usize,
}

impl From<&Game> for GameDTO {
    fn from(game: &Game) -> Self {
        GameDTO {
            players: game.players.clone(),
            vertices: game.vertices.clone(),
            tiles: game.tiles.clone(),
            harbors: game.harbors.clone(),
            robber_tile: game.robber_tile,
            current_player: game.current_player,
            dev_card_pool: game.dev_card_pool.clone(),
            turn_phase: game.turn_phase,
            game_phase: game.game_phase,
            setup_placement: game.setup_placement,
            largest_army_owner: game.largest_army_owner,
            longest_road_owner: game.longest_road_owner,
            longest_road_length: game.longest_road_length,
        }
    }
}
