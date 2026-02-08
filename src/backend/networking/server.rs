use std::collections::HashMap;
use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_ecs::message::MessageReader;

use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode,
        ConnectionLostEvent,
        EndpointAddrConfiguration,
        ServerEndpointConfiguration,
        QuinnetServer,
    },
    shared::ClientId,
};

use crate::backend::{game::{Game, RoadBuildingMode}, networking::rendezvous::RendezvousServer};
use crate::backend::networking::protocol::*;
use crate::backend::networking::bootstrap;
use crate::backend::networking::config::ConnectionMode;

#[derive(Resource)]
pub struct ServerGame {
    pub game: Game,
}

#[derive(Resource, Default)]
pub struct ServerPlayers {
    pub players: HashMap<ClientId, usize>,
    pub next_player_id: usize,
}

pub fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
    mut server_game: ResMut<ServerGame>,
    mut players: ResMut<ServerPlayers>,
) {
    server.endpoint_mut().set_default_channel(0);

    let client_ids: Vec<_> = server.endpoint_mut().clients().into_iter().collect();

    for client_id in client_ids {
        loop {
            let payload = {
                let endpoint = server.endpoint_mut();
                endpoint.try_receive_payload(client_id, 0)
            };

            let Some(payload) = payload else { break };

            let Ok(msg) = bincode::deserialize::<ClientMessage>(&payload) else {
                continue;
            };

            match msg {
                ClientMessage::Join => {
                    let id = players.next_player_id;
                    players.next_player_id += 1;
                    players.players.insert(client_id, id);

                    let reply = ServerMessage::Confirmation { player: id as u8 };
                    let payload = bincode::serialize(&reply).unwrap();

                    let _ = server
                        .endpoint_mut()
                        .try_send_payload(client_id, payload);
                }

                ClientMessage::BuildRoad { player_id, vertex1, vertex2 } => {
                    if server_game.game
                        .build_road(player_id as usize, vertex1, vertex2, RoadBuildingMode::Normal)
                        .is_ok()
                    {
                        broadcast_state(&mut server, &server_game);
                    }
                }

                ClientMessage::BuildSettlement { player_id, vertex_id } => {
                    if server_game.game
                        .build_settlement(player_id as usize, vertex_id)
                        .is_ok()
                    {
                        broadcast_state(&mut server, &server_game);
                    }
                }

                ClientMessage::BuildCity { player_id, vertex_id } => {
                    if server_game.game
                        .build_city(player_id as usize, vertex_id)
                        .is_ok()
                    {
                        broadcast_state(&mut server, &server_game);
                    }
                }

                ClientMessage::RollDice { .. } => {
                    server_game.game.roll_dice();
                    broadcast_state(&mut server, &server_game);
                }

                ClientMessage::EndTurn { .. } => {
                    server_game.game.end_turn();
                    broadcast_state(&mut server, &server_game);
                }

                ClientMessage::ChatMessage { message } => {
                    let msg = ServerMessage::ChatMessage { message };
                    let payload = bincode::serialize(&msg).unwrap();

                    let endpoint = server.endpoint_mut();
                    for c in endpoint.clients() {
                        let _ = endpoint.try_send_payload(c, payload.clone());
                    }
                }

                ClientMessage::Disconnect => {
                    players.players.remove(&client_id);
                    let _ = server.endpoint_mut().disconnect_client(client_id);
                }

                _ => {}
            }
        }
    }
}

fn broadcast_state(server: &mut QuinnetServer, game: &ServerGame) {
    let dto = GameDTO::from(&game.game);
    let msg = ServerMessage::GameStateUpdate { game: dto };
    let payload = bincode::serialize(&msg).unwrap();

    let endpoint = server.endpoint_mut();
    for c in endpoint.clients() {
        let _ = endpoint.try_send_payload(c, payload.clone());
    }
}

pub fn handle_server_events(
    mut events: MessageReader<ConnectionLostEvent>,
    mut server: ResMut<QuinnetServer>,
    mut players: ResMut<ServerPlayers>,
) {
    for ev in events.read() {
        players.players.remove(&ev.id);
        let _ = server.endpoint_mut().disconnect_client(ev.id);
    }
}

pub fn start_server(mut commands: Commands, mut server: ResMut<QuinnetServer>) {
    /*let join_code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    */
    let join_code = "ABC123".to_string();

    let rendezvous = RendezvousServer::new("0.0.0.0:4000");
    rendezvous.run_in_thread("0.0.0.0:4000");
    commands.insert_resource(rendezvous);

    let server_addr = bootstrap::host(ConnectionMode::LAN, &join_code);

    server
        .start_endpoint(ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_ip(
                Ipv4Addr::new(0, 0, 0, 0),
                6000,
            ),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: server_addr.to_string(),
            },
            defaultables: Default::default(),
        })
        .unwrap();

    let game = Game::new(vec![
        "Player 0".into(),
        "Player 1".into(),
        "Player 2".into(),
        "Player 3".into(),
    ]);

    commands.insert_resource(ServerGame { game });
    println!("Server started at {}", server_addr);
}