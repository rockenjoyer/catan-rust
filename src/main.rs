use bevy::prelude::*;
use bevy::window::{Window, WindowMode};

use std::cell::RefCell; //shared ownership pointer for the data that is thread-unsafe (RNG inside game.rs)
use std::rc::Rc; //mutability for "Game" while using Rc

use catan_rust::backend::game::Game;
use catan_rust::frontend::bevy::FrontendPlugin;

fn main() {
    //building a bevy app, creating the game state and registering the plugins
    let game = Rc::new(RefCell::new(Game::new(vec![
        "Name1", "Name2", "Name3", "Name4",
    ])));

    App::new()
        //default plugins and window configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                title: "The Settlers of Catan - Rust Edition".to_string(),
                ..default()
            }),
            ..default()
        }))
        //our defined frontend plugin for UI
        .add_plugins(FrontendPlugin)
        //inserting the game state as a non-send resource (main thread only)
        .insert_non_send_resource(game.clone())
        .run();
}
