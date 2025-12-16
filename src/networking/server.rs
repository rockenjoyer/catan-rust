use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{Mutex, Notify, broadcast, mpsc};
use std::collections::HashMap;
use std::sync::Arc;
use serde_json;

pub mod message;
pub mod game_server;

use message::GameMessage;
use game_server::GameServer;

use crate::message::ServerMessage;

/// Handles client connections and inputs
/// Numbers joined clients (1-4)
/// Broadcasts basic messages to client (transmitter only)
async fn handle_client(
    stream: TcpStream,
    server: Arc<Mutex<GameServer>>,
    clients: Arc<Mutex<HashMap<u8, broadcast::Sender<GameMessage>>>>,
    player: u8,
) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let (tx, mut rx) = broadcast::channel(32);

    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(player, tx);
    }


    if let Err(e) = write_msg(&mut writer, &GameMessage::Confirmation { player }).await {
        eprintln!("Failed to broadcast confirmation message to player {}: {}", player, e);
        return;
    }

    broadcast_msg(&clients, GameMessage::Join { player }).await;

    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<GameMessage>();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(e) = out_tx.send(msg) {
                eprintln!("Failed to forward message to client {}: {}", player, e);
                break;
            }
        }
    });

    tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if let Err(e) = write_msg(&mut writer, &msg).await {
                eprintln!("Failed to write message: {}", e);
                break;
            }
        }
    });

    /*let mut buf = vec![0u8; 1024];
    loop {
        let n = match reader.read(&mut buf).await {
            Ok(0) => {
                println!("Player {} disconnected (connection closed)", player);
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprint!("Player {} disconnected (error: {})", player, e);
                break;
            }
        };

        let msg: GameMessage = match serde_json::from_slice(&buf[..n]) {
            Ok(m) => m,
            Err(e) => {
                eprint!("Player {} sent invalid message: {}", player, e);
                buf = vec![0u8; 1024];
                continue;
            }
        };
        buf = vec![0u8; 1024];

        match msg {
            GameMessage::TurnOver { player } => {
                let mut server_lock = server.lock().await;
                if server_lock.is_current_player(player) {
                    server_lock.next_turn();
                    if let Some(current_player) = server_lock.get_current_player() {
                        broadcast_msg(&clients, GameMessage::Turn { player: server_lock.current_player }).await;
                    }
                }
            },
            GameMessage::Something { player } => {
                broadcast_msg(&clients, GameMessage::Something { player }).await;
            },
            GameMessage::Disconnect { player } => {
                println!("Player {} disconnected", player);
                broadcast_msg(&clients, GameMessage::Disconnect { player }).await;
                break;
            }
            _ => {}
        }
    }*/

    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        match serde_json::from_str::<GameMessage>(&line) {
            Ok(msg) => {
                match msg {
                    GameMessage::TurnOver { player: msg_player } => {
                        let current_player = {
                            let mut server_lock = server.lock().await;
                            if server_lock.is_current_player(msg_player) {
                                server_lock.next_turn();
                                server_lock.get_current_player()
                            } else {
                                None
                            }
                        };
                        if let Some(current_player) = current_player {
                            broadcast_msg(&clients, GameMessage::Turn { player: current_player }).await;
                        }
                    }
                    GameMessage::Something { player: msg_player } => {
                        broadcast_msg(&clients, GameMessage::Something { player: msg_player }).await;
                    }
                    GameMessage::Disconnect { player: msg_player } => {
                        println!("Player {} disconnected", msg_player);
                        broadcast_msg(&clients, GameMessage::Disconnect { player: msg_player }).await;
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Player {} sent invalid message: {}", player, e);
                continue;
            }
        };
    }

    {
        let mut clients_lock =  clients.lock().await;
        let mut server_lock = server.lock().await;
        clients_lock.remove(&player);
        server_lock.free_slot(player);
    }
}

/// Serializes given message in json then writes it
async fn write_msg<W>(writer: &mut W, msg: &GameMessage) -> tokio::io::Result<()>
where
    W: tokio::io::AsyncWriteExt + Unpin,
{
    let msg_json = serde_json::to_string(msg)?;
    writer.write_all(msg_json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}

/// Broadcasts message to all connected clients
async fn broadcast_msg(clients: &Arc<Mutex<HashMap<u8, broadcast::Sender<GameMessage>>>>, msg: GameMessage) {
    let clients_lock = clients.lock().await;
    for (_, tx) in clients_lock.iter() {
        let _ = tx.send(msg.clone());
    }
}

/// Starts the server locally (127.0.0.1:8080)
/// To try: cargo run --bin server 
/// Run "cargo run --bin client" in new terminal to test if client is connecting correctly
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is running on 127.0.0.1:8080");
    println!("Lobby created");

    let server = Arc::new(Mutex::new(GameServer::new()));
    let clients: Arc<Mutex<HashMap<u8, broadcast::Sender<GameMessage>>>> = Arc::new(Mutex::new(HashMap::new()));

    let shutdown_clients = Arc::clone(&clients);

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Error setting Ctrl-C handler");
        println!("\nShutting down server...");
        
        let shutdown_msg = GameMessage::ServerCrash;
        let clients_lock = shutdown_clients.lock().await;
        for (_, tx) in clients_lock.iter() {
            let _ = tx.send(shutdown_msg.clone());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        std::process::exit(0);
    });

    let clients_c = Arc::clone(&clients);
    let server_c = Arc::clone(&server);
    
    tokio::spawn(async move {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "start" => {
                    let mut server_lock = server_c.lock().await;
                    if !server_lock.game_started {
                        server_lock.start_game();
                        
                        let current_player = server_lock.get_current_player();
                        drop(server_lock);
                    
                        broadcast_msg(&clients_c, GameMessage::GameStart).await;
                    
                        if let Some(current_player) = current_player {
                            broadcast_msg(&clients_c, GameMessage::Turn { player: current_player }).await;
                        }
                    } else {
                        println!("Game has already started.");
                    }
                },
                "end" => {
                    let mut server_lock = server_c.lock().await;
                    if server_lock.game_started {
                        server_lock.end_game();
                        println!("Game ended.");
                        drop(server_lock);

                        broadcast_msg(&clients_c, GameMessage::GameEnd).await;
                    } else {
                        println!("Game has not started yet.")
                    }
                }
                "list players" => {
                    let mut server_lock = server_c.lock().await;
                    server_lock.list_players();
                }
                _ => {}
            };
        }
    });
    
    loop {
        let(mut stream, _) = listener.accept().await?;
        let s = Arc::clone(&server);
        let c = Arc::clone(&clients);

        tokio::spawn(async move {
            let player = s.lock().await.assign_slot();

            match player {
                Some(id) => handle_client(stream, s, c, id).await,
                None => {
                    eprintln!("No available slots");
                    let _ = stream.shutdown().await;
                }
            }
        });
    }
}
