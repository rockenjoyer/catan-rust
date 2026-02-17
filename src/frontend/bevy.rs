use bevy::prelude::*;
use bevy::window::{Window, WindowMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;

use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_quinnet::server::QuinnetServerPlugin;

use crate::backend::networking::config;
use crate::frontend::interface::{
    game_panel, info_panel, settings_panel, log_panel, main_menu, multiplayer_menu, lobby_menu,
};

use crate::frontend::system::transition::NetworkTransition;
use crate::frontend::system::{audio, camera, multiplayer::{handle_multiplayer_action, HostState}, transition::handle_network_transition};
use crate::frontend::visual::{cards, city, road, settlement, tile, dice, startscreen};

use crate::backend::networking::rendezvous::RendezvousServer;
use crate::backend::networking::server::{ServerGame, ServerPhase, ServerPlayers, handle_client_messages, handle_server_events, host_connect_as_client, start_server};
use crate::backend::networking::client::{handle_client_events, handle_server_messages, handle_terminal_messages, start_connection, ClientState};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    MultiplayerMenu,
    Lobby,
    Hosting,
    Joining,
    InGame,
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
            //background-color
            .insert_resource(ClearColor(Color::WHITE))
            .add_plugins(EguiPlugin::default())

            //audio plugin for background music
            .add_plugins(AudioPlugin::default())

            //add quinnet plugins for client and server
            .add_plugins(QuinnetClientPlugin::default())
            .add_plugins(QuinnetServerPlugin::default())

            //state management
            .init_state::<GameState>()

            //event observers for multiplayer
            .add_observer(handle_network_transition)
            .add_observer(handle_multiplayer_action)

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
            .insert_resource(game_panel::DevCardPlayState::default())
            .insert_resource(game_panel::RobberMoveState::default())
            .insert_resource(dice::DiceRollState::default())
            .insert_resource(log_panel::GameLog::default())
            .insert_resource(audio::AudioState::default())
            .insert_resource(main_menu::MainMenuState::default())
            .insert_resource(multiplayer_menu::MultiplayerMenuState::default())
            .insert_resource(config::GameMode::Local)
            .insert_resource(HostState::default())
            .insert_resource(ClientState::default())
            
            //resource for multiplayer
            .insert_resource(ServerPlayers::default())
            
            //main menu systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    startscreen::setup_startscreen_texture,
                    startscreen::setup_logo,
                    main_menu::setup_main_menu,
                    settings_panel::setup_settings,
                ).run_if(in_state(GameState::MainMenu)),
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
                ).run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                lobby_menu::setup_lobby_menu
                    .run_if(in_state(GameState::Lobby)),
            )
            .add_systems(
                OnEnter(GameState::Hosting), 
                (start_server, host_connect_as_client),
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
                    .run_if(not(in_state(GameState::MainMenu))
                    .and(not(in_state(GameState::MultiplayerMenu))))
            )
            .add_systems(
                OnEnter(GameState::Hosting),
                start_connection,
            )
            .add_systems(
                OnEnter(GameState::Joining),
                start_connection,
            )
            .add_systems(
                Update,
                (handle_client_events, handle_server_messages, handle_terminal_messages)
                    .run_if(
                        in_state(GameState::Joining)
                        .or(in_state(GameState::Hosting)) 
                    ),
            );
    }
}