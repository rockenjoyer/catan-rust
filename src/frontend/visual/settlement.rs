//"SettlementVisual" stores the logical data needed to render or interact with a settlement
//tracks which vertex the settlement occupies and which player owns it

use bevy::prelude::*;

//Component attached to entities representing settlements.
#[derive(Component)]
pub struct SettlementVisual {
    pub vertex: usize,
    pub owner_id: usize,
}
