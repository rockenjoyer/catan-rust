#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
use bevy::prelude::*;

use std::cell::RefCell; //shared ownership pointer for the data that is thread-unsafe (RNG inside game.rs)
use std::path::Path;
use std::rc::Rc; //mutability for "Game" while using Rc

use catan_rust::backend::game::Game;
use catan_rust::frontend::bevy::FrontendPlugin;

fn main() {
    //ensure relative asset paths resolve when launching from different working directory
    ensure_assets();

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

fn ensure_assets() {
    //prefer the current directory, then the exe dir, then the repo root
    if Path::new("assets").exists() {
        return;
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if dir.join("assets").exists() {
                let _ = std::env::set_current_dir(dir);
                return;
            }
        }
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if manifest_dir.join("assets").exists() {
        let _ = std::env::set_current_dir(manifest_dir);
    }
}
