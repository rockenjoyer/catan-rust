use std::collections::HashMap;
use std::time::Duration;
use std::net::Ipv4Addr;

use bevy::prelude::*;

use bevy_ecs::message::MessageReader;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::ResMut;

use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode, ConnectionLostEvent,
        EndpointAddrConfiguration, ServerEndpointConfiguration,
        QuinnetServer, endpoint::Endpoint
    },
    shared::{
        ClientId,
    }
};

use crate::networking::protocol::*;
use crate::networking::bootstrap;
use crate::networking::config::ConnectionMode;

#[derive(Resource, Debug, Clone, Default)]
pub struct Users {
    names: HashMap<ClientId, u8>,
}

#[derive(Resource, Default)]
pub struct ServerPlayers {
    pub players: HashMap<ClientId, u8>,
    pub names: HashMap<ClientId, String>,
    pub next_player_id: u8,
}


pub fn handle_client_messages(mut server: ResMut<QuinnetServer>, mut state: ResMut<ServerPlayers>) {

    let endpoint = server.endpoint_mut();
    endpoint.set_default_channel(0);

    for client_id in endpoint.clients() {
        while let Some(payload) = endpoint.try_receive_payload(client_id, 0) {
            match bincode::deserialize::<ClientMessage>(&payload) {

                Ok(ClientMessage::Join) => {
                    let player_id = state.next_player_id;
                    state.next_player_id += 1;
                    state.players.insert(client_id, player_id);

                    let player_name = format!("Player {}", player_id);
                    state.names.insert(client_id, player_name.clone());

                    let confirmation_message = ServerMessage::Confirmation { player: player_id };
                    let confirmation_payload = bincode::serialize(&confirmation_message).unwrap();
                    endpoint.try_send_payload(client_id, confirmation_payload);

                    let join_message = ServerMessage::Join { player: player_id };
                    let join_payload = bincode::serialize(&join_message).unwrap();

                    for target_client in endpoint.clients() {
                        endpoint.try_send_payload(target_client, join_payload.clone());
                    }

                    println!("Player {} joined (ClientId: {})", player_id, client_id);
                },

                Ok(ClientMessage::Something { player }) => {
                    let player_id = state.players.get(&client_id).copied().unwrap_or(0);
                    println!("Player {} did something", player_id);

                    let something_message = ServerMessage::Something { player: player_id };
                    let something_payload = bincode::serialize(&something_message).unwrap();

                    for target_client in endpoint.clients() {
                        endpoint.try_send_payload(target_client, something_payload.clone());
                    }
                },

                Ok(ClientMessage::ChatMessage { message }) => {
                    let player_id = state.players.get(&client_id).copied().unwrap_or(0);
                    let player_name = state.names.get(&client_id).cloned().unwrap_or_else(|| format!("Player {}", player_id));
                    println!("{} says: {}", player_name, message);

                    let chat_message = ServerMessage::ChatMessage { /*player: player_id*/ message: message.clone() };
                    let chat_payload = bincode::serialize(&chat_message).unwrap();

                    for target_client in endpoint.clients() {
                        endpoint.try_send_payload(target_client, chat_payload.clone());
                    }
                },

                Ok(ClientMessage::Disconnect { .. }) => {
                    if let Some(player_id) = state.players.remove(&client_id) {
                        // let player_name = player_names.remove(&client_id).unwrap_or_else(|| format!("Player {}", player_id));
                        println!("Player {} disconnected", player_id);

                        let disconnect_message = ServerMessage::Disconnect { player: u64::from(player_id) };
                        let disconnect_payload = bincode::serialize(&disconnect_message).unwrap();
                        for target_client in endpoint.clients() {
                            endpoint.try_send_payload(target_client, disconnect_payload.clone())
                        }

                        let _ = endpoint.disconnect_client(client_id);
                    }
                },

                Err(e) => {
                    println!("Failed to deserialize client message: {:?}", e)
                }

                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}

pub fn handle_server_events(
    mut connection_lost_events: MessageReader<ConnectionLostEvent>,
    mut server: ResMut<QuinnetServer>,
    mut users: ResMut<Users>,
) {
    for client in connection_lost_events.read() {
        handle_disconnect(server.endpoint_mut(), &mut users, client.id);
    }
}

pub fn handle_disconnect(endpoint: &mut Endpoint, users: &mut ResMut<Users>, client_id: ClientId) {
    
    if let Some(username) = users.names.remove(&client_id) {

        let disconnect_message = ServerMessage::Disconnect { player: client_id };
        let disconnect_payload = bincode::serialize(&disconnect_message).unwrap();
        
        endpoint
            .send_group_payload(
                users.names.keys(),
                disconnect_payload,
            )
            .unwrap();
        info!("{} disconnected", username);
    } else {
        warn!(
            "Received a Disconnect from an unknown or disconnected client: {}",
            client_id
        )
    }
}

pub fn start_server(mut server: ResMut<QuinnetServer>) {
    /*let join_code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    */
    let join_code = "ABC123".to_string();
    println!("Server started. Join code: {}", join_code);

    let server_addr = bootstrap::host(ConnectionMode::LOCAL, &join_code);
    println!("Server address: {}", server_addr);

    let _ = server.start_endpoint(
        ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_ip(Ipv4Addr::new(0, 0, 0, 0), 6000),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "84.134.111.72".to_string(),
            },
            defaultables: Default::default(),
        }
    );
    println!("Game server endpoint started on {}", server_addr);
}
