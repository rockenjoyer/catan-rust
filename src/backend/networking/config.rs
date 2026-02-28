use bevy::prelude::*;
use std::net::{SocketAddr, UdpSocket};
use std::io;
use std::fmt;

pub enum ConnectionMode {
    LOCAL,
    LAN,
    REMOTE,
}

#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameMode {
    Local,
    Multiplayer,
}

#[derive(Resource, Default)]
pub struct LanOverride {
    pub addr: Option<SocketAddr>,
}

/// WARNING: REMOTE connection functionality is currently deprecated
/// remote connections are not correctly implemented, use at your own risk
impl ConnectionMode {
    pub fn rendezvous_addr(&self, override_addr: Option<SocketAddr>) -> SocketAddr {
        match self {
            ConnectionMode::LOCAL => "127.0.0.1:4000".parse().unwrap(),
            ConnectionMode::LAN => override_addr.unwrap_or_else(|| {
                format!("{}:4000", get_local_ip().unwrap_or_else(|_| "127.0.0.1".to_string()))
                .parse()
                .unwrap()
            }),
            ConnectionMode::REMOTE => "PUBLIC_IP:4000".parse().unwrap(),
        }
    }

    /// Checks whether STUN should be used for the given connection mode.
    /// STUN is only used for REMOTE.
    pub fn use_stun(&self) -> bool {
        matches!(self, ConnectionMode::REMOTE)
    }
}

impl fmt::Display for ConnectionMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionMode::LAN => 
                 write!(f, "{}:4000", get_local_ip().unwrap_or_else(|_| "127.0.0.1".to_string())),
            _ => write!(f, ""),
        }
    }
}

/// Attempts to determine the local IP address of the machine
fn get_local_ip() -> io::Result<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_addr = socket.local_addr()?;
    Ok(local_addr.ip().to_string())
}