//bevy.rs is supposed to register rendering, camera, egui, input, etc.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::frontend::interface::{game_overlay, game_panel};
use crate::frontend::system::{camera, input};
use crate::frontend::visual::tile;

pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        //install the egui plugin, register our startup and update systems
        app
            //background-Color
            .insert_resource(ClearColor(Color::srgb(0.07, 0.03, 0.0)))
            .add_plugins(EguiPlugin::default())
            //startup runs once
            .add_systems(
                Startup,
                (
                    change_title,
                    camera::setup_camera,
                    input::setup_cursor,
                    input::input_handling,
                ),
            )
            //resource controls whether tile visuals are shown or not
            .insert_resource(tile::TileShowing::default())
            //egui pass: egui-context-related systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                game_overlay::setup_overlay,
            )
            .add_systems(bevy_egui::EguiPrimaryContextPass, game_panel::setup_panels)
            .add_systems(bevy_egui::EguiPrimaryContextPass, input::input_handling);
    }
}

fn change_title(mut window: Single<&mut Window>) {
    window.title = format!("The Settlers of Catan - Rust Edition");
}

//TO-DO: add more functions for setting up the UI (like change_title)
