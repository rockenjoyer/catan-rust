pub mod message;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::thread;
use std::sync::Arc;

use message::{GameMessage};

struct GameServer {
    players: Vec<Option<u8>>,
    current_turn: u8,
}

impl GameServer {
    fn new() -> Self {
        GameServer {
            players: vec![None; 4],
            current_turn: 1,
        }
    }

    async fn handle_client(&mut self, mut socket: TcpStream, player: u8) -> tokio::io::Result<()> {
        let mut buf = [0; 1024];

        let start_msg = GameMessage::Turn { player };
        let msg = serde_json::to_string(&start_msg).unwrap();
        socket.write_all(msg.as_bytes()).await?;

        loop {
            let n = socket.read(&mut buf).await?;
            if n == 0 {
                break;
            }

            let msg: GameMessage = match serde_json::from_slice(&buf[..n]) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("Failed to parse message: {}", e);
                    continue;
                }
            };
            
            match msg {
                GameMessage::Turn { player: _u8 } => {
                    if self.current_turn == player {
                        println!("It's your turn");
                    } else {
                        println!("It's player {}'s turn", player);
                    }
                }
                GameMessage::Join { player: _u8} => {
                    println!("Player {} joined", player);
                }
                GameMessage::Disconnect { player: _u8 } => {
                    println!("Player {} disconnected", player);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    let server = Arc::new(Mutex::new(GameServer::new()));
    let clients: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(HashMap::new()));

    println!("Hello Player 1! Lobby initiated");

    loop {
        let (stream, _) = listener.accept().await?;
        let server = Arc::clone(&server);
        let clients = Arc::clone(&clients);

        tokio::spawn(async move {
            let stream = Arc::new(Mutex::new(stream));
            let mut buf = [0; 1024];
            let server = server.lock().await;

            let player = server.players.iter().position(|p| p.is_none()).unwrap_or(0) as u8 + 1;
            let player_name = String::from(("Player ").to_owned()) + &player.to_string();

            {
                let mut stream_lock = stream.lock().await;
                {
                    let mut clients_lock = clients.lock().await;
                    clients_lock.insert(player_name.clone(), Arc::clone(&stream));
                }
            
                broadcast_msg(&clients, &format!("{} joined", player_name)).await;

                loop {
                    let n = stream_lock.read(&mut buf).await.unwrap();
                    if n == 0  {
                        break;
                    }
                    broadcast_msg(&clients, &format!("{} disconnected", player_name)).await;
                }
            }
        });
    }
}

async fn broadcast_msg(clients: &Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>, msg: &str) {
    let clients_lock = clients.lock().await;
    for (_, client) in clients_lock.iter() {
        let mut client_lock = client.lock().await;
        let _ = client_lock.write_all(msg.as_bytes()).await;
    }
}