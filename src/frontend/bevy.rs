use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;

use crate::frontend::interface::{
    game_panel, info_panel, rules_panel, settings_panel, volume_panel,
};
use crate::frontend::system::{camera, input};
use crate::frontend::visual::{cards, city, road, settlement, tile};

pub struct FrontendPlugin;

impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        //install the egui plugin, register our startup and update systems
        app
            //background-color
            .insert_resource(ClearColor(Color::WHITE))
            .add_plugins(EguiPlugin::default())
            //audio plugin for background music
            .add_plugins(AudioPlugin::default())
            //startup runs once
            .add_systems(
                Startup,
                (
                    play_background_music,
                    camera::setup_camera,
                    input::setup_cursor,
                ),
            )
            .add_systems(
                Update,
                (
                    input::input_handling,
                    tile::setup_tile_textures,
                    road::setup_road_textures,
                    settlement::setup_settlement_textures,
                    city::setup_city_textures,
                    cards::setup_cards_textures,
                ),
            )
            //egui pass: egui-context-related systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    info_panel::setup_info,
                    rules_panel::setup_rules,
                    settings_panel::setup_settings,
                    volume_panel::setup_volume,
                    game_panel::setup_game,
                ),
            );
    }
}

//doens't work yet...
fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    //load the audio
    let music = asset_server.load("audio/background_music.wav");
    audio.play(music).looped().with_volume(0.5);
}