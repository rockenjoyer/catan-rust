use bevy::prelude::*;
use bevy::window::{Window, WindowMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;

use crate::frontend::interface::{
    game_panel, info_panel, rules_panel, settings_panel, volume_panel,
};
use crate::frontend::system::{audio, camera, input};
use crate::frontend::visual::{banner, cards, city, icons, road, settlement, tile};

pub struct FrontendPlugin;

impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        //install the egui plugin, register our startup and update systems
        app
            //window configuration
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    title: "The Settlers of Catan - Rust Edition".to_string(),
                    ..default()
                }),
                ..default()
            }))
            //background-color
            .insert_resource(ClearColor(Color::WHITE))
            .add_plugins(EguiPlugin::default())
            //audio plugin for background music
            .add_plugins(AudioPlugin::default())
            //startup runs once
            .add_systems(
                Startup,
                (
                    audio::play_background_music,
                    camera::setup_camera,
                    input::setup_cursor,
                ),
            )
            .add_systems(
                Update,
                (
                    input::input_handling,
                    banner::setup_banner_textures,
                    icons::setup_icon_textures,
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
