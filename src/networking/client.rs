use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::process;
use std::sync::Arc;

pub mod message;
use message::GameMessage;


/// Handles the client connection attempt to the given server
/// Server here is localhost: 127.0.0.1:8080
/// 
/// Sends an outbound request to the server (currently only local connections possible)
/// If client connection to server is achieved: handles messaging and inputs from client to server
async fn run_client() -> tokio::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let mut lines = reader.lines();
    let confirmation_line = lines.next_line().await?.unwrap();
    let confirmation_msg: GameMessage = serde_json::from_str(&confirmation_line)?;

    let mut assigned_player: Option<u8> = None;

    match confirmation_msg {
        GameMessage::Confirmation { player } => {
            println!("You are Player {}", player);
            assigned_player = Some(player);
        }
        _ => return Ok(()),
    };

    let is_my_turn = Arc::new(Mutex::new(false));
    let game_started = Arc::new(Mutex::new(false));

    let is_my_turn_c = Arc::clone(&is_my_turn);
    let game_started_c = Arc::clone(&game_started);

    tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if let Ok(msg) = serde_json::from_str::<GameMessage>(&line) {
                match msg {
                    GameMessage::GameStart => {
                        println!("Game started!");
                        *game_started_c.lock().await = true;
                    }
                    GameMessage::Confirmation { player } => {
                        assigned_player = Some(player);
                        println!("You are Player {}", player);
                    },
                    GameMessage::Join { player } => {
                        println!("Player {} joined", player)
                    },
                    GameMessage::GameEnd => {
                        println!("Game ended!");
                        *game_started_c.lock().await = false;
                    }
                    GameMessage::Turn { player } => {
                        if let Some(assigned_player_value) = assigned_player {
                            if player == assigned_player_value {
                                println!("It's your turn");
                                *is_my_turn_c.lock().await = true;
                            } else {
                                println!("It's Player {}'s turn", player);
                                *is_my_turn_c.lock().await = false;
                            }
                        }
                    },
                    GameMessage::TurnOver { player } => {
                        println!("Player {} finished their turn", player)
                    },
                    GameMessage::Something { player } => {
                        println!("Player {} did something", player)
                    },
                    GameMessage::Disconnect { player } => {
                        println!("Player {} disconnected", player)
                    },
                    GameMessage::ServerCrash => {
                        eprintln!("Connection to server lost. Disconnecting...");
                        process::exit(0);
                        //eprintln!("If you haven't been disconnected yet, press: Ctrl + C to do so immeadietly");
                    }
                    _ => {}
                }
            }
        }
    });

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Some(assigned_player_value) = assigned_player {
            let is_my_turn_lock = is_my_turn.lock().await;
            let game_started_lock = game_started.lock().await;
        
            match input {
                "disconnect" => {
                    if let Err(e) = write_msg(&mut writer, &GameMessage::Disconnect { player: assigned_player_value }).await {
                        eprintln!("Failed to send 'disconnect' message: {}", e);
                    }
                    break;
                }
                "help" => {
                    println!("Available commands: ");
                    println!("If it's your turn: 'do something' or 'finished'");
                    println!("Otherwise: 'disconnect'");
                    continue;
                }
                "status" => {
                    println!("Game hasn't started yet. Wait for the host to start the game.");
                    continue;
                }
                _ => {}
            }

            if *game_started_lock {
                match input {
                    "status" => {
                        println!("Game has started. You're player {}", assigned_player_value);
                    }
                    _ => {}
                }       
            } else {
                println!("Game has not started yet. Wait for the game to start.");
                continue;
            }

            if *is_my_turn_lock {
                match input {
                    "finished" => {
                        if let Err(e) = write_msg(&mut writer, &GameMessage::TurnOver { player: assigned_player_value }).await {
                            eprintln!("Failed to send 'turn over' message: {}", e);
                        }
                        continue;
                    }
                    "do something" => {
                        if let Err(e) = write_msg(&mut writer, &GameMessage::Something { player: assigned_player_value }).await {
                            eprintln!("Failed to send 'do something' message: {}", e);
                        }
                        continue;
                    }
                    _ => println!("Invalid command; try 'help'")
                }       
            } else {
                println!("It's not your turn.");
                continue;
            }
        }
    }

    Ok(())
}

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
/// To try: cargo run --bin client (only if server is running in another terminal)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_client().await?;
    Ok(())
}
