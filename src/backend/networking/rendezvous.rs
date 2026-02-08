use bevy::prelude::Resource;
use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Resource)]
pub struct RendezvousServer {
    inner: Arc<Mutex<HashMap<String, SocketAddr>>>,
}

impl RendezvousServer {
    
    pub fn new(bind_addr: &str) -> Self {
        let inner = Arc::new(Mutex::new(HashMap::new()));

        println!("Rendezvous server created, bound to {}", bind_addr);

        Self { inner }
    }

    pub fn run_in_thread(&self, bind_addr: &str) {
        let inner = self.inner.clone();
        let bind_addr = bind_addr.to_string();

        thread::spawn(move || {
            let socket = UdpSocket::bind(&bind_addr).expect("Failed to bind rendezvous server");
            socket.set_nonblocking(true).unwrap();
            println!("Rendezvous server running on {}", bind_addr);

            let mut buf = [0u8; 256];

            loop {
                if let Ok((len, addr)) = socket.recv_from(&mut buf) {
                    let msg = String::from_utf8_lossy(&buf[..len]).trim().to_string();
                    println!("Rendezvous message from {}: {}", addr, msg);

                    if let Some(rest) = msg.strip_prefix("REGISTER ") {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        let code = parts[0];
                        let server_addr = if parts.len() > 1 {
                            parts[1].parse().unwrap_or_else(|_| SocketAddr::new(addr.ip(), 6000))
                        } else {
                            SocketAddr::new(addr.ip(), 6000)
                        };

                        inner.lock().unwrap().insert(code.to_string(), server_addr);
                        socket.send_to(b"READY", addr).ok();
                        println!("Registered host: {} => {}", code, server_addr);

                    } else if let Some(rest) = msg.strip_prefix("JOIN ") {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        let code = parts[0];

                        if let Some(server_addr) = inner.lock().unwrap().get(code) {
                            let server_addr_str = server_addr.to_string();
                            socket.send_to(server_addr_str.as_bytes(), addr).ok();
                            println!("Sent {} to client {}", server_addr_str, addr);
                        }
                    }
                }

                thread::sleep(Duration::from_millis(5));
            }
        });
    }
}
