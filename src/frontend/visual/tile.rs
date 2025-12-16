//"TileVisual" stores the data needed to render or interact with a tile.

use bevy::prelude::*;
use crate::backend::game::Resource;
use crate::backend::game::Resource::*;

//"Component": attached to tile entities containing game-relevant metadata.
#[derive(Component)]
pub struct TileVisual {
    //The tile's 6 corners.
    pub vertices: [usize; 6],
    //The resource type.
    pub resource: Resource,
    //Number token used for dice-based resources.
    pub number_token: Option<u8>,
    //Whether the tile is highlighted (e.g. hover or selection).
    pub highlighted: bool,
}

//Later, we can replace the colors with textures. This is for testing purposes.
#[allow(dead_code)]
fn tile_color(resource: Resource) -> Color {
    match resource {
        Brick  => Color::srgb(0.4, 0.3, 0.16),
        Lumber => Color::srgb(0.28, 0.45, 0.14),
        Wool   => Color::srgb(0.86, 0.92, 0.84),
        Grain  => Color::srgb(0.9, 0.8, 0.36),
        Ore    => Color::srgb(0.5, 0.5, 0.56),
        Desert => Color::srgb(0.62, 0.53, 0.3)
    }
}