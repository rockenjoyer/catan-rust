pub struct GameServer {
    pub players: Vec<Option<u8>>,
    pub current_player: u8,
}

impl GameServer {
    pub fn new() -> Self {
        GameServer {
            players: vec![None; 4],
            current_player: 1,
        }
    }

    pub fn next_turn(&mut self) {
        self.current_player = (self.current_player % 4) + 1;
    }

    pub fn is_current_player(&self, player: u8) -> bool {
        self.current_player == player
    }
}
