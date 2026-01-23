use bevy_quinnet::server::endpoint::Endpoint;
use bevy_quinnet::shared::ClientId;
use bevy_quinnet::shared::channels::ChannelId;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use crate::networking_test::{protocol::*, transport::ClientTransport};
use bevy_quinnet::client::{ConnectionClosed, QuinnetClient, connection};

use crate::networking_test::transport::*;

pub fn run_client(mut client: QuinnetClient) {
    let mut assigned_player: Option<u8> = None;
    let mut client_connection = client.connection_mut();

    // Channel for stdin input
    let (input_tx, input_rx) = std::sync::mpsc::channel::<String>();

    // Spawn input thread
    std::thread::spawn(move || loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let input = input.trim().to_string();
            if !input.is_empty() {
                let _ = input_tx.send(input);
            }
        }
    });


    loop {
        while let Some(payload_bytes) = client_connection.receive_payload(1).ok().flatten() {
            match bincode::deserialize::<ServerMessage>(&payload_bytes) {
                Ok(msg) => {
                    match msg {
                        ServerMessage::Confirmation { player } => {
                            assigned_player = Some(player);
                            println!("You are Player {}", player);
                        }

                        ServerMessage::GameStart => {
                            //game_started = true;
                            println!("Game started");
                        }

                        ServerMessage::Turn { player } => {
                            if Some(player) == assigned_player {
                                println!("Your turn");
                                //is_my_turn = true;
                            } else {
                                println!("Player {}'s turn", player);
                                //is_my_turn = false;
                            }
                        }

                        ServerMessage::ServerCrash => {
                            eprintln!("Server crashed");
                            return;
                        }

                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize server message: {:?}", e);
                }
            }
        }

        while let Ok(input) = input_rx.try_recv() {
            if let Some(player) = assigned_player {
                match input.as_str() {
                    "do" => {
                        let msg = ClientMessage::Something { player };
                        let payload = bincode::serialize(&msg).unwrap();
                        client_connection.send_payload(payload);
                    },
                    "quit" => {
                        let msg = ClientMessage::Disconnect { player };
                        let payload = bincode::serialize(&msg).unwrap();
                        client_connection.send_payload(payload);
                        println!("Quitting...");
                        return;
                    },
                    other => {
                        let msg = ClientMessage::Chat { player, message: other.to_string() };
                        let payload = bincode::serialize(&msg).unwrap();
                        client_connection.send_payload(payload); 
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(16)); 
    }
}
