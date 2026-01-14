//"CityVisual" is the component for a city entity. Works the same as "SettlementVisual"

use bevy::prelude::*;

#[derive(Component)]
pub struct CityVisual {
    pub vertex: usize,
    pub owner_id: usize,
}
