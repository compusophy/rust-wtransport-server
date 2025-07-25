use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::{broadcast, RwLock};
use tokio::io::AsyncReadExt;
use wtransport::{Endpoint, ServerConfig, Identity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Player {
    id: String,
    name: String,
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum GameMessage {
    PlayerJoined { player: Player },
    PlayerLeft { player_id: String },
    PlayerMoved { player_id: String, x: f32, y: f32 },
    ChatMessage { player_id: String, message: String },
    GameState { players: Vec<Player> },
}

type GameState = Arc<RwLock<HashMap<String, Player>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get port from environment or default to 4433
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4433".to_string())
        .parse::<u16>()
        .unwrap_or(4433);
    
    println!("Starting WebTransport server on port {}", port);
    
    // Generate self-signed certificate for development
    let identity = generate_self_signed_identity().await?;
    
    let bind_addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    
    let config = ServerConfig::builder()
        .with_bind_address(bind_addr)
        .with_identity(identity)
        .build();
    
    let server = Endpoint::server(config)?;
    println!("WebTransport server running on https://0.0.0.0:{}", port);
    
    let game_state = GameState::default();
    let (tx, _rx) = broadcast::channel::<GameMessage>(100);
    
    loop {
        let incoming_session = server.accept().await;
        let incoming_request = incoming_session.await?;
        let connection = incoming_request.accept().await?;
        
        let tx = tx.clone();
        let rx = tx.subscribe();
        let game_state = game_state.clone();
        
        tokio::spawn(handle_connection(connection, game_state, tx, rx));
    }
}

async fn generate_self_signed_identity() -> Result<Identity, Box<dyn std::error::Error + Send + Sync>> {
    // Generate a self-signed certificate using rcgen
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    
    // Convert to PEM format
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();
    
    // Create temporary files
    tokio::fs::write("cert.pem", cert_pem).await?;
    tokio::fs::write("key.pem", key_pem).await?;
    
    // Load the identity from the PEM files
    let identity = Identity::load_pemfiles("cert.pem", "key.pem").await?;
    
    // Clean up temporary files
    let _ = tokio::fs::remove_file("cert.pem").await;
    let _ = tokio::fs::remove_file("key.pem").await;
    
    Ok(identity)
}

async fn handle_connection(
    connection: wtransport::Connection,
    game_state: GameState,
    tx: broadcast::Sender<GameMessage>,
    mut _rx: broadcast::Receiver<GameMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let player_id = Uuid::new_v4().to_string();
    
    // Add player to game state
    let player = Player {
        id: player_id.clone(),
        name: format!("Player{}", &player_id[..6]),
        x: 50.0,
        y: 50.0,
    };
    
    game_state.write().await.insert(player_id.clone(), player.clone());
    
    // Notify other players
    let _ = tx.send(GameMessage::PlayerJoined { player: player.clone() });
    
    // Send current game state to new player
    let players: Vec<Player> = game_state.read().await.values().cloned().collect();
    let state_data = bincode::serialize(&GameMessage::GameState { players })?;
    
    // Send initial game state using datagram instead of stream for simplicity
    let _ = connection.send_datagram(&state_data);
    
    // Handle incoming streams
    loop {
        tokio::select! {
            // Handle incoming uni-directional streams
            stream_result = connection.accept_uni() => {
                match stream_result {
                    Ok(mut stream) => {
                        let mut data = Vec::new();
                        if stream.read_to_end(&mut data).await.is_ok() {
                            if let Ok(message) = bincode::deserialize::<GameMessage>(&data) {
                                match message {
                                    GameMessage::PlayerMoved { x, y, .. } => {
                                        if let Some(player) = game_state.write().await.get_mut(&player_id) {
                                            player.x = x;
                                            player.y = y;
                                            let _ = tx.send(GameMessage::PlayerMoved { 
                                                player_id: player_id.clone(), x, y 
                                            });
                                        }
                                    }
                                    GameMessage::ChatMessage { message, .. } => {
                                        let _ = tx.send(GameMessage::ChatMessage { 
                                            player_id: player_id.clone(), 
                                            message 
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Err(_) => break, // Connection closed
                }
            }
            // Handle incoming datagrams
            datagram_result = connection.receive_datagram() => {
                match datagram_result {
                    Ok(data) => {
                        if let Ok(message) = bincode::deserialize::<GameMessage>(&data) {
                            match message {
                                GameMessage::PlayerMoved { x, y, .. } => {
                                    if let Some(player) = game_state.write().await.get_mut(&player_id) {
                                        player.x = x;
                                        player.y = y;
                                        let _ = tx.send(GameMessage::PlayerMoved { 
                                            player_id: player_id.clone(), x, y 
                                        });
                                    }
                                }
                                GameMessage::ChatMessage { message, .. } => {
                                    let _ = tx.send(GameMessage::ChatMessage { 
                                        player_id: player_id.clone(), 
                                        message 
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(_) => break, // Connection closed
                }
            }
        }
    }
    
    // Cleanup on disconnect
    game_state.write().await.remove(&player_id);
    let _ = tx.send(GameMessage::PlayerLeft { player_id });
    
    Ok(())
}
