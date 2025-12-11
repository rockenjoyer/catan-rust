use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

pub mod message;

use message::{GameMessage};


/// Handles the client connection attempt to the given server
/// Server here is localhost: 127.0.0.1:8080
async fn run_client(player: u8) -> tokio::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let stream = Arc::new(Mutex::new(stream));

    let join_msg = GameMessage::Join { player };
    let msg = serde_json::to_string(&join_msg).unwrap(); //input given name + (player n) - identifier, unique id, auto generated string key, etc.
    
    {
        let mut stream_lock = stream.lock().await;
        stream_lock.write_all(msg.as_bytes()).await?;
    }

    let stream_clone = Arc::clone(&stream);

    tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            let mut stream_lock = stream_clone.lock().await;
            let n = match stream_lock.read(&mut buf).await {
                Ok(n) if n == 0 => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            };

            if let Ok(msg) = serde_json::from_slice::<GameMessage>(&buf[..n]) {
                match msg {
                    GameMessage::Join { player: _u8} => {
                        println!("Player {} joined", player);
                    }
                    GameMessage::Turn { player: _u8 } => {
                    println!("It's player {}'s turn", player);
                    }
                    GameMessage::TurnOver { player: _u8 } => {
                    println!("Player {} finished their turn", player);
                    }
                    GameMessage::Disconnect { player: _u8 } => {
                        println!("Player {} disconnected", player);
                    }
                    GameMessage::Confirmation { player: _u8 } => {
                        println!("Welcome, player {}", player);
                    }
                    GameMessage::Something { player: _u8 } => {
                        println!("Player {} did something", player);
                    }
                    _ => {
                        println!("Invalid");
                    }
                }
            } else {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("{}", msg);
            }
        }
    });

    // to test messaging between server, clients (temp valid inputs)
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let msg = match input {
            "finished" => {
                let turn_over_msg = GameMessage::TurnOver { player };
                serde_json::to_string(&turn_over_msg).unwrap()
            },
            "do something" => {
                let something_msg = GameMessage::Something { player };
                serde_json::to_string(&something_msg).unwrap()
            },
            "disconnect" => {
                let dc_msg = GameMessage::Disconnect { player };
                let msg = serde_json::to_string(&dc_msg).unwrap();
                let mut stream_lock = stream.lock().await;
                stream_lock.write_all(msg.as_bytes()).await?;
                break;
            },
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        let mut stream_lock = stream.lock().await;
        stream_lock.write_all(msg.as_bytes()).await?;
    }

    Ok(())
}

/// Handles the client connection attemps and tracking of connection errors
/// Basically used to implement run_client and other fn's correctly
/// To try: cargo run --bin client (only if server is running in another terminal)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
    let player1 = tokio::spawn(async {
        if let Err(e) = run_client(1).await {
            eprintln!("Client 1 error: {}", e);
        }
    });

    let player2 = tokio::spawn(async {
        if let Err(e) = run_client(2).await {
            eprintln!("Client 2 error: {}", e);
        }
    });

    let player3 = tokio::spawn(async {
        if let Err(e) = run_client(3).await {
            eprintln!("Client 3 error: {}", e);
        }
    });

    let player4 = tokio::spawn(async {
        if let Err(e) = run_client(4).await {
            eprintln!("Client 4 error: {}", e);
        }
    });

    tokio::try_join!(player1, player2, player3, player4)?;

    println!("All clients finished");*/

    let player_handles: Vec<_> = (1..=4).map(|player: u8| {
        tokio::spawn(async move {
            if let Err(e) = run_client(player).await {
                eprintln!("Client {} error: {}", player, e);
            }
        })
    }).collect();

    for handle in player_handles {
        if let Err(e) = handle.await {
            eprintln!("Client task failed: {}", e);
        }
    }
    
    println!("All clients finished");
    Ok(())
}
