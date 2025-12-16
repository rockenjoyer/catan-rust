//bevy.rs is supposed to register rendering, camera, egui, input, etc.

use bevy::{prelude::*};
use bevy_egui::{EguiPlugin};

use crate::frontend::system::{camera, render, input};
use crate::frontend::interface::{game_panel, game_overlay};

pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {

    fn build(&self, app: &mut App) {

        //Install the egui plugin, register our startup and update systems.
        app
        
            //Background-Color.
            .insert_resource(ClearColor(Color::srgb(0.15, 0.12, 0.08)))

            .add_plugins(EguiPlugin::default())

            //Startup runs once.
            .add_systems(Startup, (
                change_title, 
                camera::setup_camera, 
                input::setup_cursor, 
                input::input_handling,
                render::setup_tiles,
            ))

            //"EguiPrimaryContextPass" avoids panicking.
            .add_systems(bevy_egui::EguiPrimaryContextPass, (
                game_overlay::setup_overlay, 
                game_panel::setup_panels,
                render::sync_game,
                input::input_handling,
            ));
    }
}

fn change_title(mut window: Single<&mut Window>) {
    window.title = format!("The Settlers of Catan - Rust Edition");
}

//TO-DO: add more functions for setting up the UI (like change_title).