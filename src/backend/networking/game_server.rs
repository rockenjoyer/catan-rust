pub struct GameServer {
    pub players: Vec<Option<u8>>,
    pub current_player: u8,
    pub game_started: bool,
}

impl GameServer {
    pub fn new() -> Self {
        GameServer {
            players: vec![None; 4],
            current_player: 1,
            game_started: false,
        }
    }

    pub fn assign_slot(&mut self) -> Option<u8> {
        if self.game_started {
            return None;
        }
        for (i, slot) in self.players.iter_mut().enumerate() {
            if slot.is_none() {
                let player = i as u8 + 1;
                *slot = Some(player);
                println!("Assigned slot {} to player {}", i, player);
                return Some(player);
            }
        }
        None
    }

    pub fn free_slot(&mut self, player: u8) {
        if let Some(slot) = self.players.get_mut((player - 1) as usize) {
            *slot = None;
        }
    }
    pub fn next_turn(&mut self) {
        if !self.game_started {
            return;
        }

        let mut next_player = self.current_player;
        for _i in 0..4 {
            next_player = if next_player == 4 { 1 } else { next_player + 1 };
            if self.players[next_player as usize - 1].is_some() {
                self.current_player = next_player;
                return;
            }
        }
    }

    pub fn is_current_player(&self, player: u8) -> bool {
        self.current_player == player
    }

    pub fn get_current_player(&self) -> Option<u8> {
        if self.game_started {
            Some(self.current_player)
        } else {
            None
        }
    }

    pub fn start_game(&mut self) {
        if !self.game_started {
            self.game_started = true;

            for i in 0..4 {
                if self.players[i].is_some() {
                    self.current_player = i as u8 + 1;
                    break;
                }
            }
            println!("Game started. First player up is: Player {}", self.current_player);
        }
    }

    pub fn end_game(&mut self) {
        self.game_started = false;
        println!("Game ended.");
    }

    pub fn list_players(&mut self) {
        let all_players = self.players.clone();
        println!("All players currently players:");
        for i in all_players {
            println!("Player {:?}", i);
        }
    }
}
