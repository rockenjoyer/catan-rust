use bevy::prelude::*;
use bevy::window::{Window, WindowMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;

use crate::frontend::interface::{
    game_panel, info_panel, rules_panel, settings_panel, log_panel, main_menu,
};
use crate::frontend::system::{audio, camera};
use crate::frontend::visual::{cards, city, icons, road, settlement, tile, dice, startscreen};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

pub struct FrontendPlugin;

impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        app
            //window configuration
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    title: format!("The Settlers of Catan - Rust Edition"),
                    ..default()
                }),
                ..default()
            }))
            //background-color
            .insert_resource(ClearColor(Color::WHITE))
            .add_plugins(EguiPlugin::default())

            //audio plugin for background music
            .add_plugins(AudioPlugin::default())

            //state management
            .init_state::<GameState>()

            //startup runs once
            .add_systems(
                Startup,
                (
                    audio::play_background_music,
                    camera::setup_camera,
                ),
            )

            //resources for game state
            .insert_resource(tile::ClickedVertex::default())
            .insert_resource(game_panel::RoadBuildingState::default())
            .insert_resource(game_panel::BuildingMode::default())
            .insert_resource(dice::DiceRollState::default())
            .insert_resource(log_panel::GameLog::default())
            .insert_resource(audio::AudioState::default())
            
            //main menu systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    startscreen::setup_startscreen_texture,
                    main_menu::setup_main_menu,
                ).run_if(in_state(GameState::MainMenu)),
            )
            //egui pass: texture loading and UI systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    //texture loaders (run first)
                    tile::setup_tile_textures,
                    road::setup_road_textures,
                    cards::setup_cards_textures,
                    settlement::setup_settlement_textures,
                    city::setup_city_textures,
                    icons::setup_icon_textures,
                    
                    //UI panels (run after textures loaded)
                    info_panel::setup_info,
                    game_panel::setup_game,
                    rules_panel::setup_rules,
                    settings_panel::setup_settings,
                    log_panel::setup_log_panel,
                ).run_if(in_state(GameState::InGame)),
            );
    }
}