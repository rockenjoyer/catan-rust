use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex};
use std::collections::HashMap;
use std::sync::Arc;
use serde_json;

pub mod message;
pub mod game_server;

use message::GameMessage;
use game_server::GameServer;


pub async fn run_server(
    server: Arc<Mutex<GameServer>>,
    stream: Arc<Mutex<TcpStream>>,
    player: u8,
    clients: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>
) -> tokio::io::Result<()> {

    let mut buf = [0; 1024];

    {
        let mut stream_lock = stream.lock().await;
        let welcome_msg = format!("Welcome, Player {}", player);
        if let Err(e) = stream_lock.write_all(welcome_msg.as_bytes()).await {
            eprintln!("Failed to send welcome message to player {}: {}", player, e);
            return Err(e);
        }
    }

    /*let join_msg = GameMessage::Join { player };
    let msg = serde_json::to_string(&join_msg).unwrap();
    broadcast_msg(&clients, &msg).await;

    let server_lock = server.lock().await;
    let turn_msg = GameMessage::Turn { player: server_lock.current_player };
    let msg = serde_json::to_string(&turn_msg).unwrap();
    broadcast_msg(&clients, &msg).await;

    drop(server_lock);*/

    loop {
        let mut stream_lock = stream.lock().await;
        let n = match stream_lock.read(&mut buf).await {
            Ok(n) if n == 0 => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from player {}: {}", player, e);
                break;
            }
        };

        /*let msg: GameMessage = match serde_json::from_slice(&buf[..n]) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
                continue;
            }
        }; */
        
        match serde_json::from_slice::<GameMessage>(&buf[..n]) {
            Ok(msg) => {
                match msg {
                    GameMessage::TurnOver { player } => {
                        let mut server_lock = server.lock().await;
                        if server_lock.current_player == player {
                            server_lock.next_turn();
                            let turn_msg = GameMessage::Turn { player: server_lock.current_player };
                            let msg = match serde_json::to_string(&turn_msg) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    eprintln!("Failed to serialize turn message: {}", e);
                                    continue;
                                }
                            };
                            if let Err(e) = broadcast_msg(&clients, &msg).await {
                                eprint!("Failed to broadcast turn message: {}", e);
                            }
                        }
                    }
                    GameMessage::Disconnect { player } => {
                        // todo...
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprint!("Failed to parse message from player {}: {}", player, e);
                continue;
            }
        }
    }
    Ok(())
}

/// Starts the server locally (127.0.0.1:8080)
/// To try: cargo run --bin server
/// Run "cargo run --bin client" in new terminal to test if client is connecting correctly
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    let server = Arc::new(Mutex::new(GameServer::new()));
    let clients: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(HashMap::new()));

    println!("Lobby created; connection outgoing");

    loop {
        let (stream, _) = listener.accept().await?;
        print!("New connection established");
        let server = Arc::clone(&server);
        let clients = Arc::clone(&clients);

        tokio::spawn(async move {
            let stream = Arc::new(Mutex::new(stream));

            let player = {
                let mut server_lock = server.lock().await;
                let player = match server_lock.players.iter().position(|p| p.is_none()) {
                    Some(slot) => {
                        let player = slot as u8 + 1;
                        println!("Hello player {}", player);
                        player
                    },
                    None => {
                        eprintln!("No available player slots");
                        return;
                    }
                };

                let player_name = format!("Player {}", player);
                server_lock.players[(player - 1) as usize] = Some(player);

                { 
                    let mut clients_lock = clients.lock().await;
                    clients_lock.insert(player_name.clone(), Arc::clone(&stream));
                }

                player
            };

            if let Err(e) = run_server(Arc::clone(&server), Arc::clone(&stream), player, Arc::clone(&clients)).await {
                eprintln!("Error handling player {}: {}", player, e);
            }

            let mut server_lock = server.lock().await;
            server_lock.players[(player - 1) as usize] = None;
        });
    }
}

async fn broadcast_msg(clients: &Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>, msg: &str) -> tokio::io::Result<()> {
    let clients_lock = clients.lock().await;
    let mut disconnected_clients = Vec::new();

    for (name, client) in clients_lock.iter() {
        let mut client_lock = client.lock().await;
        if let Err(e) = client_lock.write_all(msg.as_bytes()).await {
            eprintln!("Failed to send message to clients {}: {}", name, e);
            disconnected_clients.push(name.clone());
        }
    }

    if !disconnected_clients.is_empty() {
        let mut clients_lock = clients.lock().await;
        for name in disconnected_clients {
            clients_lock.remove(&name);
        }
    }

    Ok(())
}