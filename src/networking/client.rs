pub mod message;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use message::{GameMessage};

async fn run_client(player: u8) -> tokio::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let stream = Arc::new(Mutex::new(stream));
    let mut buf = [0; 1024];

    let name = format!("player {}", player); //input given name + (player n) - identifier, unique id, auto generated string key, etc.
    {
        let mut stream_lock = stream.lock().await;
        stream_lock.write_all(name.as_bytes()).await.unwrap();
    }

    /*let join_msg = GameMessage::Join { player };
    let msg = serde_json::to_string(&join_msg).unwrap();
    stream.write_all(msg.as_bytes()).await?;*/

    let stream_clone = Arc::clone(&stream);

    tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            let mut stream_lock = stream_clone.lock().await;
            let n = stream_lock.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }
            let msg = "Player disconnected";
            println!("{}", msg);
        }
    });

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut stream_lock = stream.lock().await;
        stream_lock.write_all(input.as_bytes()).await?;
    }
}

    /*loop {
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
                GameMessage::Join { player: _u8} => {
                    println!("Player {} joined", player);
                }
                GameMessage::Turn { player: _u8 } => {
                println!("It's player {}'s turn", player);
                }
                GameMessage::Disconnect { player: _u8 } => {
                    println!("Player {} disconnected", player);
                }
                GameMessage::Confirmation { player: _u8 } => {
                    println!("Player {} joined", player);
                }
            }
        }
    Ok(())
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    /*let player2 = tokio::spawn(async {
        run_client(2).await
    });
    let player3 = tokio::spawn(async {
        run_client(3).await
    });
    let player4 = tokio::spawn(async {
        run_client(4).await
    });

    tokio::try_join!(player2, player3, player4)?;*/
    Ok(())
}