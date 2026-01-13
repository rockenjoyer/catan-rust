use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Resource {
    Brick,
    Lumber,
    Wool,
    Grain,
    Ore,
    Desert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    SetupRound1,    //clockwise
    SetupRound2,    //counter clockwise
    NormalPlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase{
    RollResources,
    Trade,
    Build,
    EndTurn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DevCard {
    Knight,
    VictoryPoint,
    Monopoly,
    RoadBuilding,
    YearOfPlenty,
}

//dev card struct (to give them an age)
#[derive(Clone, Debug, Copy)]
pub struct DevCardInstance {
    pub card: DevCard,
    pub age: usize,
}

//vertices at hex corners
#[derive(Debug)]
pub struct Vertex {
    pub id: usize,
    pub pos: (f32, f32), //added pos variable for usage as vertices on the UI board
    pub neighbors: HashSet<usize>, // neighboring vertices
}

impl Vertex {
    //creates a vertex
    pub fn new(id: usize, pos: (f32, f32)) -> Self {
        Vertex {
            id,
            pos,
            neighbors: HashSet::new(),
        }
    }

    //adds a neighboring vertex
    pub fn connect(&mut self, other: usize) {
        self.neighbors.insert(other);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub resource: Resource,
    pub number_token: Option<u8>,
    pub vertices: [usize; 6], //6 corners
}

//harbor can be generic or have a specific resource
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarborType {
    Generic,            
    Resource(Resource), 
}

#[derive(Debug, Copy, Clone)]
pub struct Harbor {
    pub loc0: usize, 
    pub loc1: usize,
    pub htype: HarborType,
}

//2 possible harbor patterns for the 19-tile map of the base game
const HARBOR_PATTERN_1: &[(usize, usize)] = &[
    (0, 5),
    (6, 7),
    (12, 22),
    (35, 36),
    (45, 46),
    (50, 51),
    (48, 49),
    (26, 40),
    (16, 17),
];
const HARBOR_PATTERN_2: &[(usize, usize)] = &[
    (10, 11),
    (1, 6),
    (4, 17),
    (27, 28),
    (39, 40),
    (47, 51),
    (52, 53),
    (37, 45),
    (22, 23),
];

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub resources: HashMap<Resource, u8>,
    pub settlements: HashSet<usize>,
    pub last_setup_settlement: Option<usize>,
    pub cities: HashSet<usize>,
    pub roads: HashSet<(usize, usize)>,
    pub dev_cards: Vec<DevCardInstance>,
    pub knights_played: u8,
    pub victory_points: u8,
}

impl Player {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            resources: HashMap::new(),
            settlements: HashSet::new(),
            last_setup_settlement: None,
            cities: HashSet::new(),
            roads: HashSet::new(),
            dev_cards: Vec::new(),
            knights_played: 0,
            victory_points: 0,
        }
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub vertices: Vec<Vertex>,
    pub tiles: Vec<Tile>,
    pub harbors: Vec<Harbor>,
    pub robber_tile: usize,
    pub current_player: usize,
    pub dev_card_pool: Vec<DevCard>,
    pub turn_phase: TurnPhase,
    pub game_phase: GamePhase,
    pub setup_placement: u8, //how many placements current player made (0-2)
    pub largest_army_owner: Option<usize>,
    pub longest_road_owner: Option<usize>,
    pub longest_road_length: usize,
    rng: rand::rngs::ThreadRng, //local random number generator
}

impl Game {
    pub fn new(player_names: Vec<&str>) -> Self {

        //local random number generator
        let mut rng = rand::rng();
        //checks if the player number is 2-4
        assert!((2..=4).contains(&player_names.len()));

        let players: Vec<Player> = player_names
            //takes list of names
            .into_iter()
            //assigns IDs
            .enumerate()
            //creates Player instances
            .map(|(id, name)| Player::new(id, name))
            //collects the Players into a vector
            .collect();

        // Generate board and harbors
        let (vertices, tiles) = Game::generate_board(&mut rng);
        let harbors = Game::generate_harbors(&mut rng);

        // Robber starts on desert tile
        let robber_tile = tiles
            .iter()
            .position(|t| t.resource == Resource::Desert)
            .expect("There must be a desert tile");

        // Dev card pool: 14 knights, 5 victory points, 2 of all others
        let mut dev_card_pool = vec![
            DevCard::Knight, DevCard::Knight, DevCard::Knight, DevCard::Knight, DevCard::Knight, 
            DevCard::Knight, DevCard::Knight, DevCard::Knight, DevCard::Knight, DevCard::Knight, 
            DevCard::Knight, DevCard::Knight, DevCard::Knight, DevCard::Knight, DevCard::VictoryPoint, 
            DevCard::VictoryPoint, DevCard::VictoryPoint, DevCard::VictoryPoint, DevCard::VictoryPoint, DevCard::Monopoly, 
            DevCard::Monopoly, DevCard::RoadBuilding, DevCard::RoadBuilding, DevCard::YearOfPlenty, DevCard::YearOfPlenty,
        ];
        dev_card_pool.shuffle(&mut rng);

        Game {
            players,
            vertices,
            tiles,
            harbors,
            robber_tile,
            current_player: 0,
            dev_card_pool,
            turn_phase: TurnPhase::RollResources,
            game_phase: GamePhase::SetupRound1,
            setup_placement: 0,
            largest_army_owner: None,
            longest_road_owner: None,
            longest_road_length: 0,
            rng,
        }
    }

    pub fn generate_board_from_coords(
        rng: &mut rand::rngs::ThreadRng,
        hex_coords: Vec<(i32, i32)>
    ) -> (Vec<Vertex>, Vec<Tile>) {

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut vertex_map: HashMap<(i32, i32), usize> = HashMap::new();
        let mut tiles: Vec<Tile> = Vec::new();

        //standard resources for 19 tiles
        let mut resource_pool = vec![
            Resource::Brick, Resource::Brick, Resource::Brick,
            Resource::Lumber, Resource::Lumber, Resource::Lumber, Resource::Lumber,
            Resource::Wool, Resource::Wool, Resource::Wool, Resource::Wool,
            Resource::Grain, Resource::Grain, Resource::Grain, Resource::Grain,
            Resource::Ore, Resource::Ore, Resource::Ore,
            Resource::Desert,
        ];
        //randomizes resources
        resource_pool.shuffle(rng);

        //standard tokens
        let mut number_pool = vec![2,3,3,4,4,5,5,6,6,
                                            8,8,9,9,10,10,11,11,12];
        //randomizes tokens
        number_pool.shuffle(rng);

        //hex tile size
        //currently set to 1.0 but can be adjusted
        let size = 1.0_f32;
        //square root of 3
        let sqrt3 = 1.7320508075688772_f32;

        //hex corner offsets
        //used to calculate corner coodinates later
        const CORNERS: [(f32, f32); 6] = [
            ( 0.0,     -1.0),
            ( 0.8660254, -0.5),
            ( 0.8660254,  0.5),
            ( 0.0,      1.0),
            (-0.8660254, 0.5),
            (-0.8660254,-0.5),
        ];

        for (i, &(q, r)) in hex_coords.iter().enumerate() {
            //gives the hex a random resource
            let resource = resource_pool[i % resource_pool.len()]; //modulo if the board has more than 19 hexes
            //gives the hex a random token number
            //desert gets None
            //2 if token pool runs out
            let number_token =
                if resource == Resource::Desert {
                    None
                } else {
                    Some(number_pool.pop().unwrap_or(2))
                };

            //turns axial coordinates into pixel coordinates
            let cx = size * (sqrt3 * q as f32 + (sqrt3 / 2.0) * r as f32);  //horizontal position of the hex center
            let cy = size * ((3.0 / 2.0) * r as f32);   //vertical position of the hex center

            let mut tile_vertex_indices = [0usize; 6];

            for (corner_i, &(dx, dy)) in CORNERS.iter().enumerate() {
                //pixel coordinates of corner vertices
                //adding the offsets to the hex center location considering size
                let vx = cx + dx * size;
                let vy = cy + dy * size;

                //converting the pixel locations into keys
                //* 1000 to consider float point precision errors
                let key = (
                    (vx * 1000.0).round() as i32,
                    (vy * 1000.0).round() as i32,
                );

                //check if corner exists in vertex map
                //if yes reuse its index
                let v_idx = if let Some(&idx) = vertex_map.get(&key) {
                    idx
                //if not add it to vertex map
                } else {
                    let idx = vertices.len();
                    vertices.push(Vertex::new(idx, (vx, vy)));
                    vertex_map.insert(key, idx);
                    idx
                };

                tile_vertex_indices[corner_i] = v_idx;
            }

            tiles.push(Tile {
                resource,
                number_token,
                vertices: tile_vertex_indices,
            });
        }

        //connects neighbors bidirectionally
        for tile in &tiles {
            for i in 0..6 {
                let v1 = tile.vertices[i];
                let v2 = tile.vertices[(i + 1) % 6];
                vertices[v1].connect(v2);
                vertices[v2].connect(v1);
            }
        }

        (vertices, tiles)
    }
    
    pub fn generate_board(rng: &mut rand::rngs::ThreadRng) -> (Vec<Vertex>, Vec<Tile>) {
        
        //hex coordinates for a normal 19 tile map
        let hex_coords = vec![
            (0,-2), (1,-2), (2,-2),
            (-1,-1), (0,-1), (1,-1), (2,-1),
            (-2,0), (-1,0), (0,0), (1,0), (2,0),
            (-2,1), (-1,1), (0,1), (1,1),
            (-2,2), (-1,2), (0,2)
        ];

        Self::generate_board_from_coords(rng, hex_coords)
    }

    pub fn generate_board_custom(
        rng: &mut rand::rngs::ThreadRng,

        //vector of coordinates for custom shaped boards
        hex_coords: Vec<(i32, i32)>,
    ) -> (Vec<Vertex>, Vec<Tile>) {
        
        Self::generate_board_from_coords(rng, hex_coords)
    }

    pub fn generate_harbors<R: Rng>(rng: &mut R) -> Vec<Harbor> {
        //choose pattern 1 or 2 at random
        let pattern = if rng.random_bool(0.5) {
            HARBOR_PATTERN_1
        } else {
            HARBOR_PATTERN_2
        };

        //make list of all 9 harbors and shuffle
        let mut harbor_types = vec![
            HarborType::Generic,
            HarborType::Generic,
            HarborType::Generic,
            HarborType::Generic,
        ];
        for res in [
            Resource::Brick,
            Resource::Lumber,
            Resource::Ore,
            Resource::Grain,
            Resource::Wool,
        ] {
            harbor_types.push(HarborType::Resource(res));
        }
        harbor_types.shuffle(rng);

        //generate harbors
        pattern
            .iter()
            .zip(harbor_types.into_iter())
            .map(|(&(loc0, loc1), htype)| Harbor {
                loc0,
                loc1,
                htype,
            })
            .collect()
    }

    //check if the current player has 10 victory points
    fn check_for_winner(&self) -> Option<usize> {
        let current_player = &self.players[self.current_player];
        if current_player.victory_points >= 10 {
            Some(current_player.id)
        } else {
            None
        }
    }

    //turn phase transitions
    pub fn next_phase(&mut self) {
        self.turn_phase = match self.turn_phase {
            TurnPhase::RollResources => TurnPhase::Trade,
            TurnPhase::Trade => TurnPhase::Build,
            TurnPhase::Build => TurnPhase::EndTurn,
            TurnPhase::EndTurn => {
                //when turn ends it moves to the next player
                //and starts with roll phase
                self.next_turn();
                TurnPhase::RollResources
            },
        };

        if let Some(winner_id) = self.check_for_winner() {
            println!("Player {} wins the game!", winner_id);
        }
    }

    //starts the turn and sets the phase to roll phase
    pub fn start_turn(&mut self) {
        self.turn_phase = TurnPhase::RollResources;
    }

    //ends the turn
    pub fn end_turn(&mut self) {
        self.turn_phase = TurnPhase::EndTurn;
    }

    //gives the turn to the next player
    pub fn next_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len(); //modulo for looping
        self.start_turn();

        //adjust age for dev cards
        for player in &mut self.players {
            for card in &mut player.dev_cards {
                card.age += 1;
            }
        }
    }

    //2d6 rolls
    pub fn roll_dice(&mut self) -> u8 {
        let die1 = self.rng.random_range(1..=6);
        let die2 = self.rng.random_range(1..=6);
        let total = die1 + die2;

        if total == 7 {
            self.handle_robber();
        } else {
            self.distribute_resources(total);
        }

        total
    }

    fn distribute_resources(&mut self, dice_roll: u8) {
        for (i, tile) in self.tiles.iter().enumerate() {
            //if the tile has the same token number as dice roll
            //if it doesnt have the robber
            if tile.number_token == Some(dice_roll) && i != self.robber_tile {
                for &vertex_idx in &tile.vertices {
                    for player in &mut self.players {
                        let mut amount = 0;
                        //+1 for settlements
                        if player.settlements.contains(&vertex_idx) { amount += 1; }
                        //+2 for cities
                        if player.cities.contains(&vertex_idx) { amount += 2; }
                        if amount > 0 {
                            //if the player doesnt have the resource already inserts a 0
                            //adds resources
                            *player.resources.entry(tile.resource).or_insert(0) += amount;
                        }
                    }
                }
            }
        }
    }


    //first part of a robber turn, takes away half of a players resources if they have too much stuff
    fn robbery(&mut self) {
        //checks for each player if they have more than 7 resources
        for player in &mut self.players {
            if player.resources.values().sum::<u8>() > 7 {
                //discarded amount is half
                let discard_amount = player.resources.values().sum::<u8>() / 2;

                let mut discarded = 0;
                //creates a vector of players resources
                let mut resource_keys: Vec<Resource> = player.resources.keys().cloned().collect();
                //randomizes resources
                resource_keys.shuffle(&mut self.rng);

                //discards untill the discard amount is fulfilled
                while discarded < discard_amount {
                    for resource in resource_keys.iter() {
                        if let Some(amount) = player.resources.get_mut(resource) {
                            if *amount > 0 {
                                *amount -= 1;
                                discarded += 1;
                            }
                        }
                    }
                }
            }
        }
    }  

    fn update_largest_army(&mut self, player_id: usize) {
        let player_knights = self.players[player_id].knights_played;

        if player_knights <3 {
            return;
        }

        match self.largest_army_owner {

            //no one owns it yet
            None => {
                self.largest_army_owner = Some(player_id);
                self.players[player_id].victory_points += 2;
                println!("Player {} gains Largest Army!", player_id);
            }

            Some(current_owner) => {
                if current_owner == player_id {
                    return; //already owns it
                }

                let current_knights = self.players[current_owner].knights_played;

                if player_knights > current_knights {
                    //remove from old owner
                    self.players[current_owner].victory_points -= 2;
                
                    //give to new owner
                    self.players[player_id].victory_points += 2;
                    self.largest_army_owner = Some(player_id);

                    println!("Player {} takes Largest Army from Player {}!", player_id, current_owner);
                }
            }
        }
    }

    //second part of a robber turn, moves the robber and allows to steal one resource (this is also the knight dev card)
    fn handle_knight(&mut self) {
        //checks for available tiles for the robber
        let available_tiles: Vec<usize> = self.tiles.iter()
            .enumerate()
            //has to move to a new tile
            .filter(|(idx, _)| *idx != self.robber_tile)
            .map(|(idx, _)| idx)
            .collect();

        //prompts the current player to choose a tile
        let current_player = &self.players[self.current_player];
        println!("Player {}: Choose a tile to place the robber:", current_player.name);
        //shows all available tiles
        for (i, tile) in available_tiles.iter().enumerate() {
            let tile_resource = &self.tiles[*tile].resource;
            println!("Option {}: Tile {} with resource {:?}", i + 1, *tile, tile_resource);
        }

        //placeholder untill implemented IO
        //choses the first available tile for simplicity
        let chosen_tile = available_tiles[0];

        //moves the robber
        self.robber_tile = chosen_tile;

        let robber_tile = &self.tiles[self.robber_tile];

        //gathers player IDs from players that can be robbed
        let robbable_players: HashSet<usize> = robber_tile.vertices.iter()
            .filter_map(|&vertex_idx| {
                self.players.iter()
                    .find(|p| p.id != self.current_player && (p.settlements.contains(&vertex_idx) || p.cities.contains(&vertex_idx)))
                    .map(|p| p.id)
            })
            .collect();

        if robbable_players.is_empty() {
            println!("No players to rob");
            return;
        }

        //prompts the current player to choose a player
        println!("Players to rob:");
        //shows available players
        for (i, player_id) in robbable_players.iter().enumerate() {
            let player = &self.players[*player_id];
            println!("Option {}: Player {}", i + 1, player.name);
        }

        //placeholder untill implemented IO
        //choses the first available player for simplicity
        let victim_id = *robbable_players.iter().next().unwrap();

        let robbed_player = &mut self.players[victim_id];

        //makes a vector of available resources to steal from the chosen player
        let available_resources: Vec<Resource> = robbed_player.resources
            .iter()
            .filter_map(|(resource, &amount)| {
                if amount > 0 {
                    Some(*resource)
                } else {
                    None
                }
            })
            .collect();

        //ends the function if no available resources
        if available_resources.is_empty() {
            println!("Player {} has no resources to steal!", robbed_player.name);
            return;
        }

        //picks random resource from the available
        let stolen_resource = *available_resources.choose(&mut self.rng).unwrap();

        //checks the targeted players Resource HashMap
        if let Some(amount) = robbed_player.resources.get_mut(&stolen_resource) {
            if *amount > 0 {
                    *amount -= 1; //takes 1 resource from the targeted player
                 
                let entry = self.players[self.current_player].resources.entry(stolen_resource).or_insert(0);
                *entry += 1; //gives 1 resource to the turn player
            }
        }
    }

    fn handle_robber(&mut self) {
        self.robbery();
        self.handle_knight();
    }


    //determines the trading ratio of each resource dependent on usable harbors
    pub fn maritime_trade_ratio(&mut self, player_id: usize, resource: Resource) -> u8 {
        let mut ratio = 4;
        let player= &mut self.players[player_id];

        for harbor in &self.harbors {
            let has_access = 
                player.settlements.contains(&harbor.loc0) 
                || player.settlements.contains(&harbor.loc1)
                || player.cities.contains(&harbor.loc0)
                || player.cities.contains(&harbor.loc1);
            
            if !has_access {continue;}

            match harbor.htype {
                HarborType::Resource(res) if res == resource => {return 2;}
                HarborType::Generic => {ratio = ratio.min(3);}
                _ => {}
            }
        }

        ratio
    }

    //regular non-player trading
    pub fn maritime_trade(&mut self, player_id: usize, offer: Resource, request: Resource) -> Result<(), &'static str> {
        let ratio = self.maritime_trade_ratio(player_id, offer);
        let player = &mut self.players[player_id];
        
        if *player.resources.get(&offer).unwrap_or(&0) < ratio {
            return Err("Not enough resources");
        }

        *player.resources.get_mut(&offer).unwrap() -= ratio;
        *player.resources.entry(request).or_insert(0) += 1;

        Ok(())
    }

    //player trade (after both players have agreed)
    pub fn player_trade(
        &mut self, 
        vendor_id: usize, 
        customer_id: usize, 
        offer: Resource, 
        amount_offer: u8, 
        request: Resource, 
        amount_request: u8
    ) -> Result<(), &'static str> {
        let (vendor, customer) = if vendor_id < customer_id {
            let (left, right) = self.players.split_at_mut(customer_id);
            (&mut left[vendor_id], &mut right[0])
        } else if vendor_id > customer_id {
            let (left, right) = self.players.split_at_mut(vendor_id);
            (&mut right[0], &mut left[customer_id])
        } else {
            return Err("vendor and customer cannot be the same");
        };

        if *vendor.resources.get(&offer).unwrap_or(&0) < amount_offer 
        || *customer.resources.get(&request).unwrap_or(&0) < amount_request {
            return Err("Not enough resources");
        }

        *vendor.resources.get_mut(&offer).unwrap() -= amount_offer;
        *vendor.resources.entry(request).or_insert(0) += amount_request;
        *customer.resources.get_mut(&request).unwrap() -= amount_request;
        *customer.resources.entry(offer).or_insert(0) += amount_offer;

        Ok(())
    }


    pub fn build_settlement(&mut self, player_id: usize, vertex: usize) -> Result<(), &'static str> {
        
        //if setup phase
        let is_setup = matches!(
            self.game_phase,
            GamePhase::SetupRound1 | GamePhase::SetupRound2
        );

        //checks if vertex is free
        if self.players.iter().any(|p| p.settlements.contains(&vertex) || p.cities.contains(&vertex)) {
            return Err("Vertex is already occupied");
        }

        //distance rule
        for neighbor in &self.vertices[vertex].neighbors {
            if self.players.iter().any(|p| p.settlements.contains(neighbor) || p.cities.contains(neighbor)) {
                return Err("Too close to another settlement or city");
            }
        }

        let is_standard = self.tiles.len() <= 19;
        let player = &mut self.players[player_id];

        //has to be conneceted to a road
        //ignore during setup phase
        if !is_setup {
            let is_connected = player.roads.iter().any(|&(x, y)| x == vertex || y == vertex);
            if !is_connected {
                return Err("Settlement must be connected to your road");
            }
        }

        //checks if under max allowed number
        if is_standard && player.settlements.len() >= 5 {
            return Err("Maximum number of settlements reached");
        }

        //doesnt allow building 2 settlements in a row during setup
        if is_setup && self.setup_placement != 0 {
            return Err("Must build road after settlement")
        }

        //ignore during setup phase
        if !is_setup {
            //needed resources
            let needed = [Resource::Brick, Resource::Lumber, Resource::Wool, Resource::Grain];
            //checks resources
            for &r in &needed {
                if player.resources.get(&r).unwrap_or(&0) < &1 { return Err("Not enough resources"); }
            }
            //removes resources
            for &r in &needed { *player.resources.get_mut(&r).unwrap() -= 1; }
        }

        //adds settlement
        player.settlements.insert(vertex);
        //adds 1 victory point
        player.victory_points += 1;

        //note last settlement during setup
        if is_setup {
            player.last_setup_settlement = Some(vertex);
        }
        Ok(())
    }

    pub fn build_city(&mut self, player_id: usize, vertex: usize) -> Result<(), &'static str> {
        //checks if its already occupied by another player
        if self.players.iter().any(|p| (p.id != player_id) && (p.settlements.contains(&vertex) || p.cities.contains(&vertex))) {
            return Err("This vertex is already occupied by another player");
        }
        let is_standard = self.tiles.len() <= 19;
        let player = &mut self.players[player_id];

        //check if there is a settlement
        if !player.settlements.contains(&vertex) {
            return Err("No settlement to upgrade on this vertex");}
        
        //cant upgrade a city
        if player.cities.contains(&vertex) {
            return Err("City already exists on this vertex");
        }

        //checks if under max allowed number
        if is_standard && player.cities.len() >= 4 {
            return Err("Maximum number of cities reached");
        }
        
        //needed resources
        let needed = [(Resource::Grain,2),(Resource::Ore,3)];
        //checks resources
        for &(r,c) in &needed {
            if player.resources.get(&r).unwrap_or(&0) < &c { return Err("Not enough resources"); }
        }
        //removes resources
        for &(r,c) in &needed { *player.resources.get_mut(&r).unwrap() -= c; }

        //removes settlement
        player.settlements.remove(&vertex);
        //adds city
        player.cities.insert(vertex);
        //adds 1 victory point
        player.victory_points += 1;
        Ok(())
    }

    fn longest_road_for_player(&self, player_id: usize) -> usize {
        let player = &self.players[player_id];

        //adjacency map of player roads
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(a, b) in &player.roads {
            adj.entry(a).or_default().push(b);
            adj.entry(b).or_default().push(a);
        }

        //recursive depth first search to find longerts path from vertex
        fn dfs(current: usize, adj: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<(usize, usize)>) -> usize {
            let mut max_len = 0;
            //if currrent vertex has neighbors
            if let Some(neighbors) = adj.get(&current) {
                for &n in neighbors {
                    //normalizes the edge
                    let edge = if current < n {(current, n)} else {(n, current)};
                    //skips edge if already visited in the current path
                    if !visited.contains(&edge) {
                        //marks the road as visited
                        visited.insert(edge);
                        //recursivelly explore the nieghbor + 1
                        max_len = max_len.max(1 + dfs(n, adj, visited));
                        //remove the edhe so other paths can use it
                        visited.remove(&edge);
                    }
                }
            }
            max_len
        }

        //try starting from each vertex
        let mut overall_max = 0;
        for &v in adj.keys() {
            let mut visited = HashSet::new();
            overall_max = overall_max.max(dfs(v, &adj, &mut visited));
        }

        overall_max
    }

    fn update_longest_road(&mut self, player_id: usize) {
        let road_length = self.longest_road_for_player(player_id);

        if road_length < 5 {
            return;
        }

        match self.longest_road_owner {
            None => {
                self.longest_road_owner = Some(player_id);
                self.players[player_id].victory_points += 2;
                self.longest_road_length = road_length;
                println!("Player {} gains Longest Road!", player_id);
            }

            Some(owner) => {
                if owner != player_id && road_length > self.longest_road_length {
                    self.players[owner].victory_points -= 2;
                    self.players[player_id].victory_points += 2;
                    self.longest_road_owner = Some(player_id);
                    self.longest_road_length = road_length;
                    println!("Player {} takes Longest Road from Player {}!", player_id, owner);
                } else {
                    //update length if same owner improves
                    if owner == player_id {
                        self.longest_road_length = road_length;
                    }
                }
            }
        }
    }

    //makes it so road (0,1) and (1,0) are the same
    fn normalize_road(a: usize, b: usize) -> (usize, usize) {
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn build_road(&mut self, player_id: usize, a: usize, b: usize) -> Result<(), &'static str> {
        
        //if setup phase
        let is_setup = matches!(
            self.game_phase,
            GamePhase::SetupRound1 | GamePhase::SetupRound2
        );
        
        let player = &mut self.players[player_id];      
        //vertices must be neighbors
        if !self.vertices[a].neighbors.contains(&b) {
            return Err("Vertices not adjacent");
        }

        //road has to be connected to a settlment, city or road
        let is_connected = 
            player.settlements.contains(&a) || player.cities.contains(&a) || 
            player.roads.iter().any(|&(x, y)| x == a || y == a) ||
            player.settlements.contains(&b) || player.cities.contains(&b) || 
            player.roads.iter().any(|&(x, y)| x == b || y == b);
        if !is_connected {
        return Err("Road must be connected to your existing infrastructure (settlement, city, or road)");
        }

        //normalize road
        let road = Game::normalize_road(a, b);

        //road already exists
        for p in &self.players {
            if p.roads.contains(&road) {
                return Err("Road already exists");
            }
        }

        let is_standard = self.tiles.len() <= 19;
        let player = &mut self.players[player_id];

        //checks if under max allowed number
        if is_standard && player.roads.len() >= 15 {
            return Err("Maximum number of roads reached");
        }

        //during setup
        //make player place settlement before road
        //road has to be connected
        if is_setup {
            let settlement = player.last_setup_settlement
                .ok_or("Must place settlement before road")?;

            if a != settlement && b != settlement {
                return Err("Road must connect to the placed settlement");
            }
        }

        //ignore during setup phase
        if !is_setup {
            //needed resources
            let needed = [Resource::Brick, Resource::Lumber];
            //checks resources
            for &r in &needed {
                if player.resources.get(&r).unwrap_or(&0) < &1 { return Err("Not enough resources"); }
            }
            //removes resources
            for &r in &needed { *player.resources.get_mut(&r).unwrap() -= 1; }
        }

        //adds road
        player.roads.insert(road);

        self.update_longest_road(player_id);

        //in setup phase
        if is_setup {
            self.setup_placement += 1;

            //if player completed only the first action (built settlement)
            if self.setup_placement == 1 {
                //same player continues
                //needs to place a road
                return Ok(());
            }

            //after road is placed
            //reset the counter for next player
            self.setup_placement = 0;

            //branches if setup round 1 or 2
            match self.game_phase {
                GamePhase::SetupRound1 => {
                    //next player in players vector
                    //left to right (clockwise)
                    self.current_player += 1;
                    //if went past last player
                    if self.current_player == self.players.len() {
                        //return to last player
                        self.current_player -= 1;
                        //switch to setup round 2
                        self.game_phase = GamePhase::SetupRound2;
                    }
                }
                GamePhase::SetupRound2 => {
                    //if back to first player
                    if self.current_player == 0 {
                        //setup is complete => begin game
                        self.game_phase = GamePhase::NormalPlay;
                    } else {
                        //move backwards to previous player
                        self.current_player -= 1;
                    }
                }
                _ => {} //safety catch
            }
        }

        Ok(())
    }


    pub fn buy_dev_card(&mut self, player_id: usize) -> Result<(), &'static str> {
        let player = &mut self.players[player_id];

        //handling resources
        let needed = [Resource::Ore, Resource::Grain, Resource::Wool];
        for &r in &needed {
            if player.resources.get(&r).unwrap_or(&0) < &1 { return Err("Not enough resources"); }
        }
        for &r in &needed { *player.resources.get_mut(&r).unwrap() -= 1; }

        //put dev card in inventory and indicate if its usable or not
        match self.dev_card_pool.pop() {
            None => return Err("Dev card pool is depleted"),
            Some(new_dev_card) => {
                match new_dev_card {
                    DevCard::VictoryPoint => player.dev_cards.push(DevCardInstance {
                        card: new_dev_card, age: 1,
                    }),
                    _ => player.dev_cards.push(DevCardInstance {
                        card: new_dev_card, age: 0
                    }),
                }
            }
        }
        Ok(())
    }

    //You can do this in any Turn Phase
    pub fn play_dev_card(&mut self, player_id: usize, card_id: usize) -> Result<(), &'static str> {
        let player = &mut self.players[player_id];

        //not allowed during setup phase
        if self.game_phase != GamePhase::NormalPlay {
            return Err("Cannot play dev cards during setup");
        }

        if card_id >= player.dev_cards.len() {
            return Err("Invalid dev card index");
        }
        
        if player.dev_cards[card_id].age < 1 {
            return Err("Cannot use this card yet");
        }
        
        let chosen_card = player.dev_cards.remove(card_id);

        match chosen_card.card {
            DevCard::Knight => {
                player.knights_played += 1;
                self.handle_knight();
                self.update_largest_army(player_id);
            }

            DevCard::VictoryPoint => {
                player.victory_points += 1;
            }

            DevCard::Monopoly => {
                //Placeholder: chosen resource defaults to Ore for now, until we have IO
                let chosen_resource = Resource::Ore;

                let mut acquired = 0;

                //collect all the resources
                for (idx, victim) in self.players.iter_mut().enumerate() {
                    if idx == player_id {
                        continue;
                    }

                    if let Some(amount) = victim.resources.remove(&chosen_resource) {
                        acquired += amount;
                    }
                }

                //give the resources to the player
                *self.players[player_id].resources.entry(chosen_resource).or_insert(0) += acquired;
            }

            DevCard::RoadBuilding => {
                //give player lumber and brick (to keep the roadbuilding function intact)
                *self.players[player_id].resources.entry(Resource::Lumber).or_insert(0) += 2;
                *self.players[player_id].resources.entry(Resource::Brick).or_insert(0) += 2;

                //Placeholder numbers until we have IO
                let r1a = 1;
                let r1b = 2;
                let r2a = 2;
                let r2b = 3;

                //force player to build roads
                self.build_road(player_id, r1a, r1b)?;
                self.build_road(player_id, r2a, r2b)?;
            }

            DevCard::YearOfPlenty => {
                //Placeholder: Defaults to Ore and Wool until we have IO
                let resource1 = Resource::Ore;
                let resource2 = Resource::Wool;

                *self.players[player_id].resources.entry(resource1).or_insert(0) += 1;
                *self.players[player_id].resources.entry(resource2).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }
}


//-------------------------
//----------TESTS----------
//-------------------------

#[cfg(test)]
mod tests {
    use super::*;

    //standard game has 19 tiles
    #[test]
    fn test_board_tile_count() {
        let game = Game::new(vec!["Alice", "Bob"]);
        // Standard Catan has 19 tiles
        assert_eq!(game.tiles.len(), 19);
    }

    //each tile has 6 corners
    #[test]
    fn test_tile_has_six_vertices() {
        let game = Game::new(vec!["Alice", "Bob"]);
        for tile in &game.tiles {
            assert_eq!(tile.vertices.len(), 6);
        }
    }

    //connections between vertices are bidirectional
    #[test]
    fn test_vertex_neighbors_bidirectional() {
        let game = Game::new(vec!["Alice", "Bob"]);
        for vertex in &game.vertices {
            for &neighbor_idx in &vertex.neighbors {
                let neighbor = &game.vertices[neighbor_idx];
                assert!(neighbor.neighbors.contains(&vertex.id),
                    "Neighbor relationship should be bidirectional");
            }
        }
    }

    //only one desert tile
    #[test]
    fn test_single_desert_tile() {
        let game = Game::new(vec!["Alice", "Bob"]);
        let desert_tiles: Vec<&Tile> = game.tiles.iter()
            .filter(|t| t.resource == Resource::Desert)
            .collect();
        assert_eq!(desert_tiles.len(), 1, "There should be exactly one desert tile");
    }

    //number tokens are valid
    #[test]
    fn test_number_tokens_valid() {
        let game = Game::new(vec!["Alice", "Bob"]);
        for tile in &game.tiles {
            if tile.resource != Resource::Desert {
                assert!(tile.number_token.is_some(), "Non-desert tiles should have number tokens");
                let token = tile.number_token.unwrap();
                assert!((2..=12).contains(&token) && token != 7,
                    "Number token should be between 2 and 12, excluding 7");
            } else {
                assert!(tile.number_token.is_none(), "Desert tile should have no number token");
            }
        }
    }

    //7 tiles
    #[test]
    fn test_small_board() {
        let mut rng = rand::rng();
        let hex_coords = vec![
            (0, 0),
            (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1),
            (1, 2),
        ];
        
        let (_vertices, tiles) = Game::generate_board_custom(&mut rng, hex_coords);
        assert_eq!(tiles.len(), 7);
        for tile in &tiles {
            assert_eq!(tile.vertices.len(), 6);
        }
    }

    //9 tiles
    #[test]
    fn test_rectangular_board() {
        let mut rng = rand::rng();
        let hex_coords = vec![
            (0,0),(1,0),(2,0),
            (0,1),(1,1),(2,1),
            (0,2),(1,2),(2,2),
        ];
        let (_vertices, tiles) = Game::generate_board_custom(&mut rng, hex_coords);
        assert_eq!(tiles.len(), 9);
        for tile in &tiles {
            assert_eq!(tile.vertices.len(), 6);
        }
    }

    #[cfg(test)]

    #[test]
    fn test_vertex_connections() {
        let mut rng = rand::rng();

        let hex_coords = vec![
            (7,-1),
            (0,0), (1,0), (7,0), (8,0),
            (-1,1), (0,1)
        ];

        let (vertices, tiles) = Game::generate_board_from_coords(&mut rng, hex_coords);

        //each tile should have 6 vertices
        for tile in &tiles {
            assert_eq!(tile.vertices.len(), 6);
        }

        //each vertex should have at least 2 neighbors
        for vertex in &vertices {
            assert!(vertex.neighbors.len() >= 2, 
                "Vertex {} has too few neighbors: {:?}", vertex.id, vertex.neighbors);
        }

        //print the total amount of vertices
        println!("Total vertices: {}", vertices.len());
        
        //for each vertex print its neighbors
        for vertex in &vertices {
            let mut neighbor_indices = vec![];
                for neighbor in &vertex.neighbors {
                neighbor_indices.push(*neighbor);
            }
            println!(
                "Vertex {} (neighbors: {:?})",
                vertex.id,
                neighbor_indices
            );
        }

        //standard 19 tile Catan coordinates
        let hex_coords = vec![
            (0,0),(1,0),(2,0),
            (-1,1),(0,1),(1,1),(2,1),
            (-2,2),(-1,2),(0,2),(1,2),(2,2),
            (-2,3),(-1,3),(0,3),(1,3),
            (-2,4),(-1,4),(0,4),
        ];

        let (vertices, tiles) = Game::generate_board_from_coords(&mut rng, hex_coords);

        //each tile should have 6 vertices
        for tile in &tiles {
            assert_eq!(tile.vertices.len(), 6);
        }

        //each vertex should have at least 2 neighbors
        for vertex in &vertices {
            assert!(vertex.neighbors.len() >= 2, 
                "Vertex {} has too few neighbors: {:?}", vertex.id, vertex.neighbors);
        }

        //print the total amount of vertices
        println!("Total vertices: {}", vertices.len());
        
        //for each vertex print its neighbors
        for vertex in &vertices {
            let mut neighbor_indices = vec![];
            // Iterate over each connection of the current vertex
            for neighbor in &vertex.neighbors {
                neighbor_indices.push(*neighbor);
            }
            println!(
                "Vertex {} (neighbors: {:?})",
                vertex.id,
                neighbor_indices
            );
        }
    }
}