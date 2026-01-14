//"RoadVisual" is the component for a road entity
//it represents an edge between two vertices and its owner

use bevy::prelude::*;

#[derive(Component)]
pub struct RoadVisual {
    pub vertices: [usize; 2],
    pub owner_id: usize,
}
