use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::thread::sleep;

pub fn host(rendezvous: &str, join_code: &str) -> (UdpSocket, SocketAddr) {
    let rendezvous_addr: SocketAddr = rendezvous.parse()
        .expect("Rendezvous must be a valid SocketAddr, e.g., 127.0.0.1:4000");

    let socket = UdpSocket::bind("0.0.0.0:5000")
        .expect("Failed to bind host socket");
    socket.set_nonblocking(true).unwrap();

    socket.send_to(format!("REGISTER {}", join_code).as_bytes(), rendezvous_addr)
        .expect("Failed to register with rendezvous");

    println!("Hosting. Join code: {}", join_code);

    let mut buf = [0u8; 256];
    let server_addr = loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf) {
            let msg = &buf[..len];
            if msg == b"READY" {
                println!("Server is ready at {}, starting NAT punch", addr);
                break addr;
            }
        }
        sleep(Duration::from_millis(10));
    };

    for _ in 0..50 {
        socket.send_to(b"punch", server_addr).ok();
        sleep(Duration::from_millis(50));
    }

    (socket, server_addr)
}

pub fn join(rendezvous: &str, join_code: &str) -> (UdpSocket, SocketAddr) {
    let rendezvous_addr: SocketAddr = rendezvous.parse()
        .expect("Rendezvous must be a valid SocketAddr, e.g., 127.0.0.1:4000");

    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Failed to bind client socket");
    socket.set_nonblocking(true).unwrap();

    socket.send_to(format!("JOIN {}", join_code).as_bytes(), rendezvous_addr)
        .expect("Failed to send join request");

    let mut buf = [0u8; 256];
    let server_addr = loop {
        if let Ok((len, _)) = socket.recv_from(&mut buf) {
            let addr: SocketAddr = String::from_utf8_lossy(&buf[..len])
                .parse()
                .expect("Failed to parse server address");
            break addr;
        }
        sleep(Duration::from_millis(10));
    };

    for _ in 0..50 {
        socket.send_to(b"punch", server_addr).ok();
        sleep(Duration::from_millis(50));
    }

    (socket, server_addr)
}
