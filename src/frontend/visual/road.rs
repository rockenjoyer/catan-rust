//"RoadVisual" is the component for a road entity
//it represents an edge between two vertices and its owner

use crate::backend::game::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct RoadVisual {
    pub vertices: [usize; 2],
    pub owner: Player, //should later only contain an owner ID, not the whole copy of "Player"
}
