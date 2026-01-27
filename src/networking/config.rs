use std::net::SocketAddr;

pub enum ConnectionMode {
    LOCAL,  
    LAN,      
    REMOTE,   
}

impl ConnectionMode {
    pub fn rendezvous_addr(&self) -> SocketAddr {
        match self {
            ConnectionMode::LOCAL => "127.0.0.1:4000".parse().unwrap(),
            ConnectionMode::LAN => "192.168.2.114:4000".parse().unwrap(),
            ConnectionMode::REMOTE => "PUBLIC_IP:4000".parse().unwrap(),
        }
    }

    pub fn use_stun(&self) -> bool {
        let use_stun = matches!(self, ConnectionMode::REMOTE);
        use_stun
    }
}
