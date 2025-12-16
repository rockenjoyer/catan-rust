use bevy::prelude::*;
use bevy::input::mouse::MouseButton;

//use bevy::window::{CursorIcon, CustomCursorImage, CustomCursor, PrimaryWindow};

use crate::frontend::visual::tile::TileVisual;

//Bevy system for handling basic mouse input.
pub fn input_handling(mouse_buttons: Res<ButtonInput<MouseButton>>, 
    //Query to get tile visuals and transforms when needed later.
    _query: Query<(&Transform, &TileVisual)>) {
    
    if mouse_buttons.just_pressed(MouseButton::Left) {
        info!("Left-click. :)");
        //TO-DO: Implement the tile selection logic.
    }

    if mouse_buttons.just_pressed(MouseButton::Right) {
        info!("Right-click. :)");
        //TO-DO: Right click to deselect? Or to show an info panel?
    }
}


pub fn setup_cursor() {
    //TO-DO: Space for a custom cursor.
}

