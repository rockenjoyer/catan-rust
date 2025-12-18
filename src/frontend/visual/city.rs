//"CityVisual" is the component for a city entity. Works the same as "SettlementVisual"

use crate::backend::game::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct CityVisual {
    pub vertex: usize,
    pub owner: Player, //should later only contain an owner ID, not the whole copy of "Player"
}
