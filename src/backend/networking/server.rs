use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use bevy_ecs::message::MessageReader;

use bevy_quinnet::client::ClientConnectionConfiguration;
use bevy_quinnet::client::connection::ClientAddrConfiguration;
use bevy_quinnet::client::certificate::CertificateVerificationMode;

use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode,
        ConnectionLostEvent,
        EndpointAddrConfiguration,
        ServerEndpointConfiguration,
        QuinnetServer,
    },
    shared::ClientId,
    client::QuinnetClient,
};

use crate::backend::{game::{Game, RoadBuildingMode, Player}, networking::{client::PendingJoin, rendezvous::RendezvousServer}};
use crate::backend::networking::protocol::*;
use crate::backend::networking::bootstrap;
use crate::backend::networking::config::ConnectionMode;

use crate::frontend::system::transition::NetworkTransition;

#[derive(Resource, PartialEq)]
pub enum ServerPhase {
    Lobby,
    InGame,
}

#[derive(Resource)]
pub struct ServerGame {
    pub game: Game,
}

#[derive(Resource, Default)]
pub struct ServerPlayers {
    pub players: HashMap<ClientId, usize>,
    pub next_player_id: usize,
    pub host: Option<ClientId>,
}

#[derive(Resource, Clone)]
pub struct JoinCode(pub String);

#[derive(Resource)]
pub struct ServerAddr(pub SocketAddr);



pub fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
    mut server_game: ResMut<ServerGame>,
    mut players: ResMut<ServerPlayers>,
    mut server_phase: ResMut<ServerPhase>,
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
                    if players.players.len() >= 4 {
                        let err = ServerMessage::ActionResult {
                            success: false,
                            message: "Lobby full".into(),
                        };

                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        return;
                    }

                    let mut id = 0;
                    while players.players.values().any(|&v| v == id) {
                        id += 1;
                    }
                    players.players.insert(client_id, id);

                    if players.host.is_none() {
                        players.host = Some(client_id);
                    }
                    
                    let player_name = format!("Player {}", id);
                    server_game.game.players.push(Player::new(id, &player_name));

                    let reply = ServerMessage::Confirmation { player: id as u8 };
                    let payload = bincode::serialize(&reply).unwrap();

                    let _ = server
                        .endpoint_mut()
                        .try_send_payload(client_id, payload);

                    let join_msg = ServerMessage::ClientConnected { player: id as u8 };
                    let join_payload = bincode::serialize(&join_msg).unwrap();

                    for &c in server.endpoint_mut().clients().iter() {
                        if c != client_id {
                            let _ = server.endpoint_mut().try_send_payload(c, join_payload.clone());
                        }
                    }

                    for (&other_client_id, &other_player_id) in players.players.iter() {
                        if other_client_id != client_id {
                            let msg = ServerMessage::ClientConnected {
                                player: other_player_id as u8,
                            };
                            let payload = bincode::serialize(&msg).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        }
                    }
                }

                ClientMessage::BuildRoad { player_id, vertex1, vertex2 } => {
                    if *server_phase != ServerPhase::InGame {
                        let err = ServerMessage::ActionResult {
                            success: false,
                            message: "Game has not started yet".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        continue;
                    }

                    if server_game.game.current_player == player_id as usize { 
                        if server_game.game.build_road(player_id as usize, vertex1, vertex2, RoadBuildingMode::Normal).is_ok() {
                            let reply = ServerMessage::Confirmation { player: player_id };
                            let reply_payload = bincode::serialize(&reply).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, reply_payload);

                            let update = ServerMessage::RoadBuilt { player_id, vertex1, vertex2 };
                            let update_payload = bincode::serialize(&update).unwrap();
                            for c in server.endpoint_mut().clients() {
                                let _ = server.endpoint_mut().try_send_payload(c, update_payload.clone());
                            }

                            broadcast_action_result(&mut server, true, "Road built".into());
                        } else {
                            let err = ServerMessage::ActionResult {
                                success: false,
                                message: "Failed to build road".into(),
                            };
                            let payload = bincode::serialize(&err).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        }
                    } else {
                        let err = ServerMessage::ActionResult { 
                            success: false, 
                            message: "Not your turn!".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                    }
                }

                ClientMessage::BuildSettlement { player_id, vertex_id } => {
                    if *server_phase != ServerPhase::InGame {
                        let err = ServerMessage::ActionResult {
                            success: false,
                            message: "Game has not started yet".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        continue;
                    }

                    if server_game.game.current_player == player_id as usize { 
                        if server_game.game.build_settlement(player_id as usize, vertex_id).is_ok() {
                            let reply = ServerMessage::Confirmation { player: player_id };
                            let reply_payload = bincode::serialize(&reply).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, reply_payload);

                            let update = ServerMessage::SettlementBuilt { player_id, vertex_id };
                            let update_payload = bincode::serialize(&update).unwrap();
                            for c in server.endpoint_mut().clients() {
                                let _ = server.endpoint_mut().try_send_payload(c, update_payload.clone());
                            }

                            broadcast_action_result(&mut server, true, "Settlement built".into());
                        } else {
                            let err = ServerMessage::ActionResult {
                                success: false,
                                message: "Failed to build settlement".into(),
                            };
                            let payload = bincode::serialize(&err).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        }
                    } else {
                        let err = ServerMessage::ActionResult { 
                            success: false, 
                            message: "Not your turn!".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                    }
                }

                ClientMessage::BuildCity { player_id, vertex_id } => {
                    if *server_phase != ServerPhase::InGame {
                        let err = ServerMessage::ActionResult {
                            success: false,
                            message: "Game has not started yet".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        continue;
                    }

                    if server_game.game.current_player == player_id as usize { 
                        if server_game.game.build_city(player_id as usize, vertex_id).is_ok() {
                            let reply = ServerMessage::Confirmation { player: player_id };
                            let reply_payload = bincode::serialize(&reply).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, reply_payload);

                            let update = ServerMessage::CityBuilt { player_id, vertex_id };
                            let update_payload = bincode::serialize(&update).unwrap();
                            for c in server.endpoint_mut().clients() {
                                let _ = server.endpoint_mut().try_send_payload(c, update_payload.clone());
                            }

                            broadcast_action_result(&mut server, true, "City built".into());
                        } else {
                            let err = ServerMessage::ActionResult {
                                success: false,
                                message: "Failed to build city".into(),
                            };
                            let payload = bincode::serialize(&err).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        }
                    } else {
                        let err = ServerMessage::ActionResult { 
                            success: false, 
                            message: "Not your turn!".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                    }
                }

                ClientMessage::RollDice { player_id } => {
                    if *server_phase != ServerPhase::InGame {
                        let err = ServerMessage::ActionResult {
                            success: false,
                            message: "Game has not started yet".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        continue;
                    }

                    if server_game.game.current_player == player_id as usize { 
                        let (die1, die2, needs_robber) = server_game.game.roll_dice();
                        let reply = ServerMessage::Confirmation { player: player_id };
                        let reply_payload = bincode::serialize(&reply).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, reply_payload);
                        
                        let update = ServerMessage::DiceRolled { die1, die2, needs_robber };
                        let update_payload = bincode::serialize(&update).unwrap();
                        for c in server.endpoint_mut().clients() {
                            let _ = server.endpoint_mut().try_send_payload(c, update_payload.clone());
                        }

                            broadcast_action_result(&mut server, true, "Dice rolled".into());
                    } else {
                        let err = ServerMessage::ActionResult { 
                            success: false, 
                            message: "Not your turn!".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                    }
                }

                ClientMessage::EndTurn { player_id } => {
                    if *server_phase != ServerPhase::InGame {
                        let err = ServerMessage::ActionResult {
                            success: false,
                            message: "Game has not started yet".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        continue;
                    }

                    if server_game.game.current_player == player_id as usize { 
                            server_game.game.end_turn();
                            let reply = ServerMessage::Confirmation { player: player_id };
                            let reply_payload = bincode::serialize(&reply).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, reply_payload);

                            let update = ServerMessage::EndedTurn { player_id };
                            let update_payload = bincode::serialize(&update).unwrap();
                            for c in server.endpoint_mut().clients() {
                                let _ = server.endpoint_mut().try_send_payload(c, update_payload.clone());
                            }

                            broadcast_action_result(&mut server, true, "Turn ended".into());
                    } else {
                        let err = ServerMessage::ActionResult { 
                            success: false, 
                            message: "Not your turn!".into(),
                        };
                        let payload = bincode::serialize(&err).unwrap();
                        let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                    }
                }

                ClientMessage::ChatMessage { message } => {
                    let msg = ServerMessage::ChatMessage { message };
                    let payload = bincode::serialize(&msg).unwrap();
                    for c in server.endpoint_mut().clients() {
                        let _ = server.endpoint_mut().try_send_payload(c, payload.clone());
                    }
                }

                ClientMessage::GameStart => {
                    if Some(client_id) == players.host {
                        if players.players.len() > 1 {
                            
                            let start_msg = ServerMessage::GameStart;
                            let payload = bincode::serialize(&start_msg).unwrap();

                            for c in server.endpoint_mut().clients() {
                                let _ = server.endpoint_mut().try_send_payload(c, payload.clone());
                            }

                            println!("Game started by host!");
                            
                            let mut phase = server_phase.as_mut();
                            *phase = ServerPhase::InGame;
                        } else {
                            let err_msg = ServerMessage::ActionResult {
                                success: false,
                                message: "Need at least one other player to start".into(),
                            };
                            let payload = bincode::serialize(&err_msg).unwrap();
                            let _ = server.endpoint_mut().try_send_payload(client_id, payload);
                        }
                    }
                }

                ClientMessage::Disconnect => {
                    if let Some(player_id) = players.players.remove(&client_id) {
                        if players.host == Some(client_id) {
                            players.host = None;
                            println!("Host disconnected, new host must be assigned.");
                            if let Some((&new_host, _)) = players.players.iter().next() {
                                players.host = Some(new_host);
                            }
                        }

                        let msg = ServerMessage::ClientDisconnected { player: player_id as u8 };
                        let payload = bincode::serialize(&msg).unwrap();
                        for c in server.endpoint_mut().clients() {
                            let _ = server.endpoint_mut().try_send_payload(c, payload.clone());
                        }

                        let _ = server.endpoint_mut().disconnect_client(client_id);
                    }
                }
                _ => {}
            }
        }
    }
}

fn broadcast_action_result(server: &mut QuinnetServer, success: bool, message: String) {
    let msg = ServerMessage::ActionResult { success, message };
    let payload = bincode::serialize(&msg).unwrap();
    for c in server.endpoint_mut().clients() {
        let _ = server.endpoint_mut().try_send_payload(c, payload.clone());
    }
}

fn broadcast_chat(server: &mut QuinnetServer, message: &str) {
    let msg = ServerMessage::ChatMessage { message: message.to_string() };
    let payload = bincode::serialize(&msg).unwrap();
    for c in server.endpoint_mut().clients() {
        let _ = server.endpoint_mut().try_send_payload(c, payload.clone());
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
    let join_code = "ABC123".to_string();
/*
    let join_code: String = rand::rng()
    .sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect();
*/
    commands.insert_resource(JoinCode(join_code.clone()));
    commands.insert_resource(PendingJoin {join_code: join_code.clone()});

    let rendezvous = RendezvousServer::new("0.0.0.0:4000");
    rendezvous.run_in_thread("0.0.0.0:4000");
    commands.insert_resource(rendezvous);

    let server_addr = bootstrap::host(ConnectionMode::LAN, &join_code);
    commands.insert_resource(ServerAddr(server_addr));

    server
        .start_endpoint(ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_ip(Ipv4Addr::new(0, 0, 0, 0), 6000),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: server_addr.to_string(),
            },
            defaultables: Default::default(),
        })
        .unwrap();

    let game = Game::new(vec!["player0", "player1", "player2", "player3"]);

    commands.insert_resource(ServerPlayers::default());
    commands.insert_resource(ServerPhase::Lobby);
    commands.insert_resource(ServerGame { game });

    println!("Server started at {}", server_addr);
}

pub fn host_connect_as_client(
    mut client: ResMut<QuinnetClient>,
    join_code: Res<JoinCode>,
    mut commands: Commands,
) {
    println!("Host attempting to join with code: {}", join_code.0);

    let server_addr = bootstrap::join(ConnectionMode::LAN, &join_code.0, None);

    println!("Game server address obtained: {}", server_addr);

    let _ = client.open_connection(ClientConnectionConfiguration {
        addr_config: ClientAddrConfiguration::from_ips(
            server_addr.ip(),
            server_addr.port(),
            "0.0.0.0".parse::<Ipv4Addr>().unwrap(),
            0,
        ),
        cert_mode: CertificateVerificationMode::SkipVerification,
        defaultables: Default::default(),
    });

    println!("Host connected to server at {}", server_addr);
    commands.trigger(NetworkTransition::EnterLobby);
}