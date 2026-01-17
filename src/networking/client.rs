use crate::networking::protocol::{ClientMessage, ServerMessage};
use crate::networking::transport::create_client;
use std::process;

pub fn run_client(addr: &str) {
    let transport = create_client(addr);

    let mut assigned_player: Option<u8> = None;
    let mut game_started = false;
    let mut is_my_turn = false;

    println!("Connected to server");

    let _ = transport.outgoing.send(ClientMessage::Join);

    loop {
        while let Ok(msg) = transport.incoming.try_recv() {
            match msg {
                ServerMessage::Confirmation { player } => {
                    assigned_player = Some(player);
                    println!("You are Player {}", player);
                }

                ServerMessage::GameStart => {
                    game_started = true;
                    println!("Game started");
                }

                ServerMessage::Turn { player } => {
                    if Some(player) == assigned_player {
                        println!("Your turn");
                        is_my_turn = true;
                    } else {
                        println!("Player {}'s turn", player);
                        is_my_turn = false;
                    }
                }

                ServerMessage::ServerCrash => {
                    eprintln!("Server crashed");
                    process::exit(0);
                }

                _ => {}
            }
        }

        let Some(player) = assigned_player else { continue };

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "disconnect" => {
                let _ = transport.outgoing.send(
                    ClientMessage::Disconnect { player }
                );
                break;
            }

            "finished" if is_my_turn && game_started => {
                let _ = transport.outgoing.send(
                    ClientMessage::TurnOver { player }
                );
            }

            "do something" if is_my_turn && game_started => {
                let _ = transport.outgoing.send(
                    ClientMessage::Something { player }
                );
            }

            _ => println!("Invalid command"),
        }
    }
}
