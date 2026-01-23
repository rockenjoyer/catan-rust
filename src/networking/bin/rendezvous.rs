use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::time::{Instant, Duration};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").expect("Failed to bind rendezvous");
    socket.set_nonblocking(true).unwrap();

    println!("Rendezvous server running on 0.0.0.0:4000");

    let mut hosts: HashMap<String, SocketAddr> = HashMap::new();
    let mut buf = [0u8; 256];

    loop {

        if let Ok((len, addr)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..len]).trim().to_string();

            if let Some(code) = msg.strip_prefix("REGISTER ") {
                hosts.insert(code.to_string(), addr);
                println!("Registered host: {} => {}", code, addr);
                socket.send_to(b"READY", addr).ok();
            }

            if let Some(code) = msg.strip_prefix("JOIN ") {
                if let Some(&host_addr) = hosts.get(code) {

                    socket.send_to(host_addr.to_string().as_bytes(), addr).ok();

                    socket.send_to(addr.to_string().as_bytes(), host_addr).ok();

                    println!(
                        "Introduced client {} <-> host {} for join code {}",
                        addr, host_addr, code
                    );
                } else {
                    println!(
                        "Client {} tried to join with unknown code {}",
                        addr, code
                    );
                }
            }

            println!("Message from {}: {}", addr, msg);
        }


        std::thread::sleep(Duration::from_millis(10));
    }
}

