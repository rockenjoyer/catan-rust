use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").expect("Failed to bind rendezvous server");
    socket.set_nonblocking(true).unwrap();

    println!("Rendezvous server running on 0.0.0.0:4000");

    let mut hosts: HashMap<String, SocketAddr> = HashMap::new();
    let mut buf = [0u8; 256];

    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..len]).trim().to_string();
            println!("Message from {}: {}", addr, msg);

            if let Some(code) = msg.strip_prefix("REGISTER ") {
                let server_addr = SocketAddr::new(addr.ip(), 6000);
                hosts.insert(code.to_string(), server_addr);

                println!("Registered host: {} => {}", code, server_addr);
                socket.send_to(b"READY", addr).ok();

            } else if let Some(code) = msg.strip_prefix("JOIN ") {
                if let Some(server_addr) = hosts.get(code) {
                    let server_addr_str = server_addr.to_string();
                    
                    println!("Sending server address {} to client {}", server_addr_str, addr);
                    socket.send_to(server_addr_str.as_bytes(), addr).ok();
                } else {
                    println!("Client {} tried to join with unknown code {}", addr, code);
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
