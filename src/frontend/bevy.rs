use bevy::prelude::*;
use bevy::window::{Window, WindowMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use bevy_wicon::WindowIconPlugin;
use std::path::PathBuf;

use crate::frontend::interface::{
    endscreen, game_panel, info_panel, log_panel, main_menu, settings_panel,
};
use crate::frontend::system::{audio, camera};
use crate::frontend::visual::{cards, city, dice, road, settlement, startscreen, tile};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    EndScreen,
}

pub struct FrontendPlugin;

impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        app
            //window configuration
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: format!("The Settlers of Catan - Rust Edition"),
                    ..default()
                }),
                ..default()
            }))
            //set window icon
            .add_plugins(WindowIconPlugin::with_path(
                resolve_window_icon_path().as_str(),
            ))
            //egui plugin for UI
            .add_plugins(EguiPlugin::default())
            //audio plugins for background music and sound effects
            .add_plugins(AudioPlugin::default())
            .add_audio_channel::<audio::MusicChannel>()
            .add_audio_channel::<audio::SoundEffectsChannel>()
            //state management
            .init_state::<GameState>()
            //startup runs once
            .add_systems(
                Startup,
                (audio::play_background_music, camera::setup_camera),
            )
            //update runs every frame
            .add_systems(
                Update,
                (
                    audio::play_click_sound,
                    audio::play_sound_on_roll,
                    audio::play_sound_on_placement,
                ),
            )
            //check for endgame condition during the game
            .add_systems(
                Update,
                (endscreen::check_for_endgame).run_if(in_state(GameState::InGame)),
            )
            //resources for game state
            .insert_resource(tile::ClickedVertex::default())
            .insert_resource(game_panel::RoadBuildingState::default())
            .insert_resource(game_panel::BuildingMode::default())
            .insert_resource(game_panel::DevCardPlayState::default())
            .insert_resource(game_panel::RobberMoveState::default())
            .insert_resource(game_panel::BuildEffectsState::default())
            .insert_resource(dice::DiceRollState::default())
            .insert_resource(log_panel::GameLog::default())
            .insert_resource(audio::AudioState::default())
            .insert_resource(main_menu::MainMenuState::default())
            .insert_resource(settings_panel::SettingsPanelState::default())
            .insert_resource(endscreen::EndscreenState::default())
            //main menu systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    startscreen::setup_startscreen_texture,
                    startscreen::setup_logo,
                    main_menu::setup_main_menu,
                    settings_panel::setup_settings,
                )
                    .run_if(in_state(GameState::MainMenu)),
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
                    //UI panels (run after textures loaded)
                    info_panel::setup_info,
                    game_panel::setup_game,
                    settings_panel::setup_settings,
                    log_panel::setup_log_panel,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            //endscreen systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    startscreen::setup_startscreen_texture,
                    startscreen::setup_logo,
                    endscreen::setup_endscreen,
                    settings_panel::setup_settings,
                )
                    .run_if(in_state(GameState::EndScreen)),
            );
    }
}

//try common locations so icons resolve in both terminal and .exe runs
fn resolve_window_icon_path() -> String {
    let relative = PathBuf::from("assets/game/icon.ico");

    let mut possible = Vec::new();
    possible.push(relative.clone());

    //check the current working directory (terminal)
    if let Ok(cwd) = std::env::current_dir() {
        possible.push(cwd.join(&relative));
    }

    //check the executable directory (.exe)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            possible.push(dir.join(&relative));
        }
    }

    //return the first path that exists or fallback to relative
    possible
        .into_iter()
        .find(|path| path.exists())
        .unwrap_or(relative)
        .to_string_lossy()
        .to_string()
}
