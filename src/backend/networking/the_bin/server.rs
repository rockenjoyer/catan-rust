use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy_quinnet::server::QuinnetServerPlugin;
use catan_rust::backend::networking::{
    server::{
        handle_client_messages, handle_server_events,
        start_server, ServerPlayers
    },
    client::Users,
};

fn start_server() {
    App::new()
        .add_plugins((
            ScheduleRunnerPlugin::default(),
            LogPlugin::default(),
            QuinnetServerPlugin::default(),
        ))
        .insert_resource(Users::default())
        .insert_resource(ServerPlayers::default())
        .add_systems(Startup, start_server)
        .add_systems(Update, (handle_client_messages, handle_server_events))
        .run();
}