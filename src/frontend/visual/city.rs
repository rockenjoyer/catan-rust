//"CityVisual" is the component for a city entity. Works the same as "SettlementVisual".

use bevy::prelude::*;
use crate::backend::game::Player;

#[derive(Component)]
pub struct CityVisual {
    pub vertex: usize,
    pub owner: Player 
    //Should later only contain an owner ID, not the whole copy of "Player".
} 
