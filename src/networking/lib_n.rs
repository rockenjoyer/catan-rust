pub mod server;
pub mod client;

pub use server::*;
pub use client::*;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

pub struct ServerPlugin;

pub struct ClientPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<ServerEvent>>();
        app.add_systems(PreUpdate, Self::update_system.run_if(resource_exists::<Server>));
        app.add_systems(
            PreUpdate,
            Self::emit_server_events_system
                .in_set(RenetReceive)
                .run_if(resource_exists::<Server>)
                .after(Self::update_system),
        );
    }
}

impl ServerPlugin {
    pub fn update_system(mut server: ResMut<Server>, time: Res<Time>) {
        server.update(time.delta());
    }

    pub fn emit_server_events_system(mut server: ResMut<Server>, mut server_messages: MessageWriter<ServerEvent>) {
        while let Some(event) = server.get_event() {
            server_messages.write(event);
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, Self::update_system.run_if(resource_exists::<Client>));
    }
}

impl ClientPlugin {
    pub fn update_system(mut client: ResMut<Client>, time: Res<Time>) {
        client.update(time.delta());
    }
}

pub fn client_connected(client: Option<Res<Client>>) -> bool {
    match client {
        Some(client) => client.is_connected(),
        None => false,
    }
}

pub fn client_disconnected(client: Option<Res<Client>>) -> bool {
    match client {
        Some(client) => client.is_disconnected(),
        None => true,
    }
}

pub fn client_connecting(client: Option<Res<Client>>) -> bool {
    match client {
        Some(client) => client.is_connecting(),
        None => false,
    }
}

pub fn client_just_connected(mut last_connected: Local<bool>, client: Option<Res<Client>>) -> bool {
    let connected = client.map(|client| client.is_connected()).unwrap_or(false);
    let just_connected = !*last_connected && connected;

    *last_connected = connected;
    just_connected
}

pub fn client_just_disconnected(mut last_connected: Local<bool>, client: Option<Res<Client>>) -> bool {
    let disconnected = client.map(|client| client.is_disconnected()).unwrap_or(true);
    let just_disconnected = *last_connected && disconnected;
    
    *last_connected = !disconnected;
    just_disconnected
}