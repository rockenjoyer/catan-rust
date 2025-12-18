//"TileVisual" stores the data needed to render or interact with a tile

use crate::backend::game::Resource;
use crate::backend::game::Resource::*;
use bevy::prelude::*;
use bevy_egui::egui;

//"Component": attached to tile entities containing game-relevant metadata
#[derive(Component)]
pub struct TileVisual {
    //the tile's 6 corners
    pub vertices: [usize; 6],
    //the resource type
    pub resource: Resource,
    //number token used for dice-based resources
    pub number_token: Option<u8>,
    //whether the tile is highlighted (e.g. hover or selection)
    pub highlighted: bool,
}

//resource to track whether tile visuals are shown or not
#[derive(Resource)]
pub struct TileShowing {
    pub enabled: bool,
}

impl Default for TileShowing {
    fn default() -> Self {
        TileShowing { enabled: true }
    }
}

//later, we can replace the colors with textures, this is for testing purposes
#[allow(dead_code)]
pub fn tile_color(resource: Resource) -> egui::Color32 {
    match resource {
        Brick => egui::Color32::from_rgb(182, 105, 43),
        Lumber => egui::Color32::from_rgb(45, 75, 35),
        Wool => egui::Color32::from_rgb(225, 224, 207),
        Grain => egui::Color32::from_rgb(242, 208, 97),
        Ore => egui::Color32::from_rgb(130, 130, 130),
        Desert => egui::Color32::from_rgb(218, 202, 78),
    }
}
