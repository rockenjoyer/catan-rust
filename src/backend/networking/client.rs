use std::collections::HashMap;
use std::time::Duration;
use std::thread::{self, sleep};
use std::net::IpAddr;

use rand::{distr::Alphanumeric, Rng};
use tokio::sync::mpsc;

use bevy_ecs::system::ResMut;
use bevy::{
    prelude::{Commands, Deref, DerefMut, Resource, Res},
    app::AppExit,
    ecs::message::{MessageReader, MessageWriter},
};

use bevy_quinnet::{
    client::{
        certificate::CertificateVerificationMode,
        connection::{ClientAddrConfiguration, ConnectionEvent, ConnectionFailedEvent},
        ClientConnectionConfiguration, QuinnetClient
    },
    shared::ClientId,
};

use crate::backend::{
    networking::{
        protocol::*,
        bootstrap,
        config::{ConnectionMode, LanOverride},
    },
    game::{
        Game,
        RoadBuildingMode,
    }
};

use crate::frontend::system::{
    chat::ChatState,
    transition::NetworkTransition,
};

#[derive(Resource, Clone)]
pub struct PendingJoin {
    pub join_code: String,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct Users {
    self_id: ClientId,
    names: HashMap<ClientId, String>,
}

#[derive(Resource, Debug, Default)]
pub struct ClientState {
    pub assigned_player: Option<u8>,
    pub users: HashMap<u8, String>,
    pub game: Option<Game>,
    pub messages: Vec<String>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct TerminalReceiver(mpsc::Receiver<String>);

pub fn on_app_exit(
    app_exit_events: MessageReader<AppExit>,
    mut client: ResMut<QuinnetClient>,
) {
    let disconnect_message = "disconnected";
    let disconnect_payload = bincode::serialize(&disconnect_message).unwrap();

    if !app_exit_events.is_empty() {
        client
            .connection_mut()
            .send_payload(disconnect_payload)
            .unwrap();

        sleep(Duration::from_secs_f32(0.1));
    }
}

pub fn handle_server_messages(
    mut state: ResMut<ClientState>,
    mut client: ResMut<QuinnetClient>,
    mut commands: Commands,
    mut chat_state: ResMut<ChatState>,
) {
    if !client.is_connected() {
        return;
    }

    let mut client_connection = client.connection_mut();

    while let Some(payload_bytes) = client_connection.try_receive_payload(0) {
        match bincode::deserialize::<ServerMessage>(&payload_bytes) {
            Ok(msg) => match msg {
                ServerMessage::Confirmation { player } => {
                    if state.assigned_player.is_none() {
                        state.assigned_player = Some(player);
                        commands.trigger(NetworkTransition::EnterLobby);
                    }

                    state.users.insert(player, format!("Player {}", player));
                    println!("You are Player {}", player);
                }
                ServerMessage::GameStart => {
                    println!("Game started");

                    commands.trigger(NetworkTransition::EnterGame);
                }
                ServerMessage::Turn { player } => {
                    if Some(player) == state.assigned_player {
                        println!("Your turn");
                    } else {
                        println!("Player {}'s turn", player);
                    }
                }
                ServerMessage::ServerCrash => {
                    eprintln!("Server crashed");
                }
                ServerMessage::ChatMessage { message } => {
                        println!("[Chat]: {}", message);
                        state.messages.push(message.clone());
                        chat_state.messages.push(message);
                }
                ServerMessage::ClientConnected { player } => {
                    state.users.insert(player, format!("Player {}", player));
                    println!("Player {} joined", player);
                }
                ServerMessage::ClientDisconnected { player } => {
                    if let Some(name) = state.users.remove(&player) {
                        println!("{} left", name);
                    }
                }
                ServerMessage::SettlementBuilt { player_id, vertex_id } => {
                    if let Some(game) = &mut state.game {
                        let _ = game.build_settlement(player_id as usize, vertex_id);
                    }
                }
                ServerMessage::RoadBuilt { player_id, vertex1, vertex2 } => {
                    if let Some(game) = &mut state.game {
                        let _ = game.build_road(player_id as usize, vertex1, vertex2, RoadBuildingMode::Normal);
                    }
                }
                ServerMessage::ActionResult { success, message } => {
                    if success {
                        println!("Action '{}' succeeded.", message);
                    } else {
                        println!("Action '{}' failed.", message);
                    }
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Failed to deserialize server message: {:?}", e);
            }
        }
    }
}

pub fn handle_terminal_messages(
    mut terminal_messages: ResMut<TerminalReceiver>,
    mut app_exit_events: MessageWriter<AppExit>,
    mut client: ResMut<QuinnetClient>,
    _state: Res<ClientState>,
) {
    while let Ok(message) = terminal_messages.try_recv() {
        match message.as_str() {
            "quit" => {
                let msg = ClientMessage::Disconnect;
                let payload = bincode::serialize(&msg).unwrap();
                client.connection_mut().try_send_payload(payload);
                app_exit_events.write(AppExit::Success);
            }
            other => {
                let msg = ClientMessage::ChatMessage {
                    message: other.to_string(),
                };
                let payload = bincode::serialize(&msg).unwrap();
                client.connection_mut().try_send_payload(payload);
            }
        }
    }
}

pub fn start_terminal_listener(mut commands: Commands) {
    let (tx, rx) = mpsc::channel::<String>(100);

    thread::spawn(move || loop {
        let mut buffer = String::new();
        if std::io::stdin().read_line(&mut buffer).is_ok() {
            let input = buffer.trim().to_string();
            if !input.is_empty() {
                tx.try_send(input).unwrap();
            }
        }
    });

    commands.insert_resource(TerminalReceiver(rx));
}

pub fn handle_client_events(
    mut connection_events: MessageReader<ConnectionEvent>,
    mut connection_failed_events: MessageReader<ConnectionFailedEvent>,
    mut client: ResMut<QuinnetClient>,
) {
    if !connection_events.is_empty() {
        let username: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        println!("--- Joining with name: {}", username);
        println!("--- Type 'quit' to disconnect");

        let join_message = ClientMessage::Join;
        let join_payload = bincode::serialize(&join_message).unwrap();

        client.connection_mut().send_payload(join_payload).unwrap();
        connection_events.clear();
    }

    for ev in connection_failed_events.read() {
        println!("Failed to connect: {:?}", ev.err);
    }
}

pub fn initialize_game_state(
    mut state: ResMut<ClientState>,
) {
    if state.game.is_none() {
        state.game = Some(Game::new(vec![
            "Player 0",
            "Player 1",
            "Player 2",
            "Player 3",
        ]));
    }
}

pub fn start_connection(
    mut client: ResMut<QuinnetClient>,
    pending: Option<Res<PendingJoin>>,
    override_addr: Option<Res<LanOverride>>,
    mut commands: Commands,
) {
    let Some(pending) = pending else {
        println!("No PendingJoin found. Skipping connection.");
        return;
    };

    let join_code = "ABC123"; // &pending.join_code;

    println!("Attempting to join with code: {}", join_code);

    let override_socket_addr = override_addr.and_then(|res| res.addr);
    let server_addr = bootstrap::join(ConnectionMode::LAN, join_code, override_socket_addr);

    println!("Game server address obtained: {}", server_addr);

    let _ = client.open_connection(ClientConnectionConfiguration {
        addr_config: ClientAddrConfiguration::from_ips(
            server_addr.ip(),
            server_addr.port(),
            "0.0.0.0".parse::<IpAddr>().unwrap(),
            0,
        ),
        cert_mode: CertificateVerificationMode::SkipVerification,
        defaultables: Default::default(),
    });

    println!("Client attempting to connect to server at {}", server_addr);

    commands.trigger(NetworkTransition::EnterLobby);
}

