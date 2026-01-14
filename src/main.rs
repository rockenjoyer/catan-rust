use bevy::prelude::*;

//shared ownership pointer for the data that is thread-unsafe (RNG inside game.rs)
use std::cell::RefCell;
use std::rc::Rc; //mutability for "Game" while using Rc

use catan_rust::backend::game::Game;
use catan_rust::backend::networking2::server;
use catan_rust::frontend::bevy::FrontendPlugin;

fn main() {
    //building a bevy app, creating the game state and registering the frontend plugin
    //example game setup
    let game = Rc::new(RefCell::new(Game::new(vec!["x", "y"])));

    App::new()
        .add_plugins(DefaultPlugins)
        //inserting the game state as a "non-send resource" into bevy so that systems can borrow it on the main thread
        .insert_non_send_resource(game.clone())
        .add_plugins(FrontendPlugin)
        .run();
}
