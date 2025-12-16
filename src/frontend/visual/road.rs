//"RoadVisual" is the component for a road entity.
//It represents an edge between two vertices and its owner.

use bevy::prelude::*;
use crate::backend::game::Player;

#[derive(Component)]
pub struct RoadVisual {
    pub vertices: [usize; 2],
    pub owner: Player
    //Should later only contain an owner ID, not the whole copy of "Player".
}
