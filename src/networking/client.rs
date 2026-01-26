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
    ecs::{
        message::{MessageReader, MessageWriter},
    },
};

use bevy_quinnet::{
    client::{
        certificate::CertificateVerificationMode,
        connection::{ClientAddrConfiguration, ConnectionEvent, ConnectionFailedEvent},
        ClientConnectionConfiguration, QuinnetClient
    },
    shared::ClientId,
};

use crate::networking::protocol::*;
use crate::networking::bootstrap;

#[derive(Resource, Debug, Clone, Default)]
pub struct Users {
    self_id: ClientId,
    names: HashMap<ClientId, String>,
}

#[derive(Resource, Default)]
pub struct ClientState {
    pub assigned_player: Option<u8>,
    pub users: HashMap<u8, String>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct TerminalReceiver(mpsc::Receiver<String>);

pub fn on_app_exit(app_exit_events: MessageReader<AppExit>, mut client: ResMut<QuinnetClient>) {
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

pub fn handle_server_messages(mut state: ResMut<ClientState>, mut client: ResMut<QuinnetClient>) {

    let mut client_connection = client.connection_mut();

    while let Some(payload_bytes) = client_connection.try_receive_payload(1) {
            match bincode::deserialize::<ServerMessage>(&payload_bytes) {
                Ok(msg) => {
                    match msg {
                        ServerMessage::Confirmation { player } => {
                            state.assigned_player = Some(player);
                            state.users.insert(player, format!("Player {}", player));                
                            
                            println!("You are Player {}", player);
                        },
                        ServerMessage::GameStart => {
                            println!("Game started");
                        },
                        ServerMessage::Turn { player } => {
                            if Some(player) == state.assigned_player {
                                println!("Your turn");
                            } else {
                                println!("Player {}'s turn", player);
                            }
                        },
                        ServerMessage::ServerCrash => {
                            eprintln!("Server crashed");
                            return;
                        },
                        ServerMessage::ChatMessage { message } => {
                            //let player_name = users.get(&player).unwrap_or(&format!("Player {}", player)).to_string();
                            println!("> {}", message);

                        },
                        ServerMessage::ClientConnected { player } => {
                            state.users.insert(player, format!("Player {}", player));
                            println!("Player {} joined", player);
                        },
                        ServerMessage::ClientDisconnected { player } => {
                            if let Some(player_name) = state.users.remove(&player) {
                                println!("{} left", player_name);
                            } else {
                                println!("Player {} left", player);
                            }
                        },
                        _ => {}
                    }
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
    state: Res<ClientState>,
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
                let msg = ClientMessage::ChatMessage { message: other.to_string() };
                let payload = bincode::serialize(&msg).unwrap();
                client.connection_mut().try_send_payload(payload);
            }
        }
    }
}

pub fn start_terminal_listener(mut commands: Commands) {
    let (from_terminal_sender, from_terminal_receiver) = mpsc::channel::<String>(100);

    thread::spawn(move || loop {
        let mut buffer = String::new();
        if std::io::stdin().read_line(&mut buffer).is_ok() {
            let input = buffer.trim_end().to_string();
            if !input.is_empty() {
                from_terminal_sender
                .try_send(buffer.trim_end().to_string())
                .unwrap();
            }
        }
    });

    commands.insert_resource(TerminalReceiver(from_terminal_receiver));
}

pub fn handle_client_events(
    mut connection_events: MessageReader<ConnectionEvent>,
    mut connection_failed_events: MessageReader<ConnectionFailedEvent>,
    mut client: ResMut<QuinnetClient>,
) {
    if !connection_events.is_empty() {
        // We are connected
        let username: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        println!("--- Joining with name: {}", username);
        println!("--- Type 'quit' to disconnect");

        let join_message = ClientMessage::Join;
        let join_payload = bincode::serialize(&join_message).unwrap();

        client
            .connection_mut()
            .send_payload(join_payload)
            .unwrap();

        connection_events.clear();
    }
    for ev in connection_failed_events.read() {
        println!(
            "Failed to connect: {:?}, make sure the chat-server is running.",
            ev.err
        );
    }
}

pub fn start_connection(mut client: ResMut<QuinnetClient>) {
    let join_code = std::env::args().nth(1).expect("join code");
    println!("Attempting to join with code: {}", join_code);

    let server_addr = bootstrap::join("127.0.0.1:4000", &join_code);
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
}
