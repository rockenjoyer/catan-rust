//bevy.rs is supposed to register camera, egui, input, etc.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::frontend::interface::{
    game_panel, info_panel, rules_panel, settings_panel, volume_panel,
};
use crate::frontend::system::{camera, input};
use crate::frontend::visual::tile;

pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        //install the egui plugin, register our startup and update systems
        app
            //background-color
            .insert_resource(ClearColor(Color::srgb(0.66, 0.58, 0.57)))
            //resource controls whether tile visuals are shown or not
            .insert_resource(tile::TileShowing::default())
            .add_plugins(EguiPlugin::default())
            //startup runs once
            .add_systems(
                Startup,
                (change_title, camera::setup_camera, input::setup_cursor),
            )
            .add_systems(Update, (input::input_handling, tile::setup_tile_textures))
            //egui pass: egui-context-related systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    info_panel::setup_info,
                    rules_panel::setup_rules,
                    settings_panel::setup_settings,
                    volume_panel::setup_volume,
                    input::input_handling,
                    game_panel::setup_game,
                ),
            );
    }
}

fn change_title(mut window: Single<&mut Window>) {
    window.title = format!("The Settlers of Catan - Rust Edition");
}
//TO-DO: add more functions for setting up the UI (like change_title)
