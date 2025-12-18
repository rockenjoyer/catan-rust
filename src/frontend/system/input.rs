use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::frontend::visual::tile::TileVisual;

//bevy system for handling basic mouse input
pub fn input_handling(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    //query to get tile visuals and transforms when needed later
    _query: Query<(&Transform, &TileVisual)>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        info!("Left-click. :)");
        //TO-DO: implement the tile selection logic
    }

    if mouse_buttons.just_pressed(MouseButton::Right) {
        info!("Right-click. :)");
        //TO-DO: right click to deselect or to show an info panel?
    }
}

pub fn setup_cursor() {
    //TO-DO: space for a custom cursor
}
