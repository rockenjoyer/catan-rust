use bevy::prelude::*;

use std::cell::RefCell; //shared ownership pointer for the data that is thread-unsafe (RNG inside game.rs)
use std::rc::Rc; //mutability for "Game" while using Rc

use catan_rust::backend::game::Game;
use catan_rust::frontend::bevy::FrontendPlugin;

fn main() {
    //building a bevy app, creating the game state and registering our bevy FrontendPlugin
    let game = Rc::new(RefCell::new(Game::new(vec![
        "Name1", "Name2", "Name3", "Name4",
    ])));

    App::new()
        .insert_non_send_resource(game)
        //our defined frontend plugin for UI
        .add_plugins(FrontendPlugin)
        .run();
}
