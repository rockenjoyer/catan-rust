use bevy::prelude::*;
use bevy::window::{Window, WindowMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use bevy_wicon::WindowIconPlugin;
use std::path::PathBuf;

use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_quinnet::server::{QuinnetServerPlugin, QuinnetServer};
use crate::frontend::bevy::config::LanOverride;

use crate::backend::networking::config::{self};
use crate::frontend::interface::{
    endscreen, game_panel, info_panel, log_panel, main_menu, settings_panel, multiplayer_menu, lobby_menu,
};
use crate::frontend::system::chat::render_chat_ui;
use crate::frontend::system::{audio, camera, multiplayer::*, chat::*};
use crate::frontend::visual::{cards, city, dice, road, settlement, startscreen, tile};


use crate::backend::networking::server::{ServerGame, ServerPhase, ServerPlayers, handle_client_messages, handle_server_events, host_connect_as_client, start_server, ServerAddr};
use crate::backend::networking::client::{handle_client_events, handle_server_messages, start_connection, ClientState, initialize_game_state};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    MultiplayerMenu,
    Lobby,
    Hosting,
    Joining,
    LocalInGame,
    MultiplayerInGame,
    EndScreen,
}

#[derive(Resource, Clone, PartialEq)]
pub enum GameMode {
    Local,
    Multiplayer,
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

            //add quinnet plugins for client and server
            .add_plugins(QuinnetClientPlugin::default())
            .add_plugins(QuinnetServerPlugin::default())

            //state management
            .init_state::<GameState>()

            //event observers for multiplayer
            .add_observer(handle_multiplayer_action)

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
                (endscreen::check_for_endgame).run_if(in_state(GameState::LocalInGame).or(in_state(GameState::MultiplayerInGame)))
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
            .insert_resource(multiplayer_menu::MultiplayerMenuState::default())
            .insert_resource(config::GameMode::Local)
            .insert_resource(HostState::default())
            .insert_resource(ClientState::default())
            .insert_resource(GameMode::Local)
            
            //resource for multiplayer
            .insert_resource(ServerPlayers::default())
            .init_resource::<ChatState>()
            .init_resource::<LanOverride>()
            .insert_resource(GameStartOrigin::default())
            
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
            //multiplayer menu system
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                multiplayer_menu::setup_multiplayer_menu
                    .run_if(in_state(GameState::MultiplayerMenu)),
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
                ).run_if(in_state(GameState::LocalInGame).or(in_state(GameState::MultiplayerInGame)))
            )
            .add_systems(
                OnEnter(GameState::LocalInGame),
                initialize_game_state.run_if(|mode: Res<GameMode>| *mode == GameMode::Local),
            )
            .add_systems(
                OnEnter(GameState::MultiplayerInGame),
                initialize_game_state.run_if(|mode: Res<GameMode>| *mode == GameMode::Multiplayer),
            )
            /*
            .add_systems(
                OnEnter(GameState::MultiplayerInGame),
                start_terminal_listener.run_if(not(resource_exists::<TerminalReceiver>))
            )
            */
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                lobby_menu::setup_lobby_menu
                    .run_if(in_state(GameState::Lobby)),
            )
            .add_systems(
                OnEnter(GameState::Hosting), 
                (
                    start_server.run_if(|host_state: Res<HostState>| host_state.is_host), 
                    host_connect_as_client
                        .after(start_server)
                        .run_if(|host_state: Res<HostState>| host_state.is_host)
                        .run_if(resource_exists::<ServerAddr>)
                        .run_if(resource_exists::<ClientState>),
                    ),
            )
            .add_systems(
                OnEnter(GameState::Joining),
                start_connection.run_if(in_state(GameState::Joining)),
            )
            .add_systems(
                Update,
                handle_server_events
                    .run_if(|server_game: Option<Res<ServerGame>>| server_game.is_some())
                    .run_if(in_state(GameState::Hosting)),
            )
            .add_systems(
                Update,
                handle_client_messages
                    .run_if(resource_exists::<ServerGame>)
                    .run_if(resource_exists::<ServerPhase>)
                    .run_if(resource_exists::<QuinnetServer>)
                    .run_if(not(in_state(GameState::MainMenu))
                    .and(not(in_state(GameState::MultiplayerMenu))))
            )
            .add_systems(
                Update,
                (
                    handle_client_events, 
                    handle_server_messages
                )
                    .run_if(
                        in_state(GameState::Joining)
                        .or(in_state(GameState::Hosting))
                        .or(in_state(GameState::Lobby))
                        .or(in_state(GameState::MultiplayerInGame)) 
                    ),
            )
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                render_chat_ui
                    .run_if(in_state(GameState::MultiplayerInGame))
            )
            /*
            .add_systems(
                Update,
                handle_terminal_messages
                    .run_if(resource_exists::<TerminalReceiver>)
                    .run_if(in_state(GameState::MultiplayerInGame)),
            )
            */
            //endscreen systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    startscreen::setup_startscreen_texture,
                    startscreen::setup_logo,
                    endscreen::setup_endscreen,
                    settings_panel::setup_settings,
                    audio::play_win_sound,
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
