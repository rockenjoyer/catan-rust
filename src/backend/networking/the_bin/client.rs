use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy_quinnet::client::QuinnetClientPlugin;
//use bevy_quinnet::client::client_connected;
use catan_rust::backend::networking::{
    client::{
        start_terminal_listener, start_connection,
        handle_client_events, handle_terminal_messages,
        handle_server_messages, on_app_exit, Users, ClientState
    }
};

fn start_client() {
    App::new()
        .add_plugins((
            ScheduleRunnerPlugin::default(),
            LogPlugin::default(),
            QuinnetClientPlugin::default(),
        ))
        .insert_resource(Users::default())
        .insert_resource(ClientState::default())
        .add_systems(Startup, (start_terminal_listener, start_connection))
        .add_systems(
            Update,
            (
                handle_client_events,
                (handle_terminal_messages, handle_server_messages)//.run_if(client_connected),
            ),
        )
        .add_systems(PostUpdate, on_app_exit)
        .run();
}
