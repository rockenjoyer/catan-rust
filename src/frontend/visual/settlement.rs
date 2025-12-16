//"SettlementVisual" stores the logical data needed to render or interact with a settlement.
//Tracks which vertex the settlement occupies and which player owns it.

use bevy::prelude::*;
use crate::backend::game::Player;

//Component attached to entities representing settlements.
#[derive(Component)]
pub struct SettlementVisual {
    pub vertex: usize,
    pub owner: Player
    //Should later only contain an owner ID, not the whole copy of "Player".
}
