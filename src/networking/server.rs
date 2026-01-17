use crate::networking::protocol::{ClientMessage, ServerMessage};
use crate::networking::transport::create_server;
use crate::networking::game_server::GameServer;

pub fn run_server() {
    let transport = create_server();
    let mut server = GameServer::new();

    println!("Server running");

    loop {
        if let Ok(msg) = transport.incoming.recv() {
            match msg {
                ClientMessage::Join => {
                    if let Some(player) = server.assign_slot() {
                        let _ = transport.outgoing.send(
                            ServerMessage::Confirmation { player }
                        );

                        let _ = transport.outgoing.send(
                            ServerMessage::Join { player }
                        );
                    }
                }

                ClientMessage::TurnOver { player } => {
                    if server.game_started && server.is_current_player(player) {
                        server.next_turn();

                        if let Some(next) = server.get_current_player() {
                            let _ = transport.outgoing.send(
                                ServerMessage::Turn { player: next }
                            );
                        }
                    }
                }

                ClientMessage::Something { player } => {
                    let _ = transport.outgoing.send(
                        ServerMessage::Something { player }
                    );
                }

                ClientMessage::Disconnect { player } => {
                    server.free_slot(player);

                    let _ = transport.outgoing.send(
                        ServerMessage::Disconnect { player }
                    );
                }

                ClientMessage::GameStart => {
                    if !server.game_started {
                        server.start_game();

                        let _ = transport.outgoing.send(
                            ServerMessage::GameStart
                        );

                        if let Some(player) = server.get_current_player() {
                            let _ = transport.outgoing.send(
                                ServerMessage::Turn { player }
                            );
                        }
                    }
                }

                ClientMessage::GameEnd => {
                    server.end_game();
                    let _ = transport.outgoing.send(
                        ServerMessage::GameEnd
                    );
                }
            }
        }
    }
}
