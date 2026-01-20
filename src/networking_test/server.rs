use bevy_quinnet::server::endpoint::Endpoint;
use bevy_quinnet::shared::ClientId;
use bevy_quinnet::shared::channels::ChannelId;
use bytes::Bytes;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::net::SocketAddr;
use crate::networking_test::protocol::*;
use crate::networking_test::transport::ServerTransport;
use bincode;

pub fn run_server(mut endpoint: Endpoint) {
    let mut players: HashMap<ClientId, u8> = HashMap::new();
    let mut socket_map: HashMap<ClientId, SocketAddr> = HashMap::new();
    let mut next_player_id: u8 = 1;
    endpoint.set_default_channel(1);
    
    println!("Server running");

    let mut last_tick = Instant::now();

    loop {

        // configure to check for incoming payloads - ref.: try_receive_from_async().ok()

        for client_id in endpoint.clients() {
            if let Some(payload) = endpoint.try_receive_payload(client_id, 1) {
                match bincode::deserialize::<ClientMessage>(&payload) {
                    Ok(ClientMessage::Join) => {
                        let player_id = next_player_id;
                        next_player_id += 1;

                        players.insert(client_id, player_id);


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

                    Ok(ClientMessage::Something { .. }) => {
                        let player_id = players.get(&client_id).copied().unwrap_or(0);
                        println!("Player {} did something", player_id);

                        let something_message = ServerMessage::Something { player: player_id };
                        let something_payload = bincode::serialize(&something_message).unwrap();
                        for target_client in endpoint.clients() {
                            endpoint.try_send_payload(target_client, something_payload.clone());
                        }
                    },

                    Ok(ClientMessage::Chat { player, message }) => {
                        let player_id = players.get(&client_id).copied().unwrap_or(0);
                        println!("Player {} says: {}", player_id, message);

                        let chat_message = ServerMessage::Chat { player: player_id, message };
                        let chat_payload = bincode::serialize(&chat_message).unwrap();
                        for target_client in endpoint.clients() {
                            endpoint.try_send_payload(target_client, chat_payload.clone());
                        }
                    },

                    Ok(ClientMessage::Disconnect { .. }) => {
                        if let Some(player_id) = players.remove(&client_id) {
                            println!("Player {} disconnected", player_id);

                            let disconnect_message = ServerMessage::Disconnect { player: player_id };
                            let disconnect_payload = bincode::serialize(&disconnect_message).unwrap();
                            for target_client in endpoint.clients() {
                                endpoint.try_send_payload(target_client, disconnect_payload.clone());
                            }

                            endpoint.disconnect_client(client_id);
                        }
                    },

                    _ => {}
                }

            }
        }

        std::thread::sleep(Duration::from_millis(16)); 
    }
}

fn broadcast(msg: &ServerMessage, endpoint: &mut Endpoint) {
    let payload = bincode::serialize(msg).unwrap();
    endpoint.try_broadcast_payload(payload);
}
