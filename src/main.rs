use bevy::prelude::*;

//Shared ownership pointer for the data that is thread-unsafe (RNG inside game.rs).
use std::rc::Rc; 
use std::cell::RefCell; //Mutability for "Game" while using Rc.

use catan_rust::frontend::bevy::FrontendPlugin;
use catan_rust::backend::game::Game;

fn main() {

    //Building a Bevy App, creating the game state and registering the frontend plugin.

    let game = Rc::new(RefCell::new(Game::new(vec!["x", "y"])));
    
    App::new()
        .add_plugins(DefaultPlugins)

        //Inserting the game state as a "non-send resource" into bevy so that systems
        //can borrow it on the main thread (won't work otherwise).
        .insert_non_send_resource(game.clone())
        .add_plugins(FrontendPlugin)
        .run();
} 
