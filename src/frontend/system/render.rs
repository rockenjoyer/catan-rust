use bevy::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::backend::game::Game;
use crate::frontend::visual::{tile::TileVisual, road::RoadVisual, settlement::SettlementVisual, city::CityVisual};

//This function will iterate the logical Game tiles inside game.rs and spawn an entity of "TileVisual".
pub fn setup_tiles(mut _commands: Commands, _game: NonSend<Rc<RefCell<Game>>>, _tiles: Query<&TileVisual>) {

    //TO-DO: Spawn tile visuals.
}

//We have to synchronize the game state from game.rs to our other frontend visuals (Settlements, Cities, Roads).
//Queries are used to avoid spawning duplicates.
pub fn sync_game(_commands: Commands, game: NonSend<Rc<RefCell<Game>>>,
    _settlements: Query<&SettlementVisual>, 
    _cities: Query<&CityVisual>, 
    _roads: Query<&RoadVisual>) {

    //Borrow the game to read current placement of settlements, cities and roads.
    let _game = &*game.borrow();

    //TO-DO: iterate the game state and update visuals accordingly.
}  

pub fn roll_dice_ui() {
    //TO-DO: Implement a UI element to roll dice.
}
