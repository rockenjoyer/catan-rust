use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::backend::networking::config::ConnectionMode;
use crate::backend::networking::stun_request;

/// Binds a UDP socket and registers the host with a rendezvous server using the provided join_code
/// Waits for a "READY" confirmation before returning the server address for NAT punch-through
///
/// If ConnectionMode::REMOTE, it retrieves the public address from the open google STUN server and 
/// sends it to the rendezvous server; however currently Public hosting is not supported
/// 
/// Currently join_code is always: ABC123
pub fn host(mode: ConnectionMode, join_code: &str) -> SocketAddr {
    let rendezvous_addr: SocketAddr = mode.rendezvous_addr(None);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind host socket");

    println!("Rendezvous address: {}", rendezvous_addr);

    socket.set_nonblocking(true).unwrap();

    let addr = if mode.use_stun() {
        println!("Getting public address via STUN...");

        let public_addr = stun_request::get_public_addr(&socket, "stun.l.google.com", 19302)
            .expect("Failed to get public address");

        println!("Public address: {}", public_addr);

        socket.send_to(format!("REGISTER {} {}", join_code, public_addr).as_bytes(), rendezvous_addr)
            .expect("Failed to register with rendezvous");

        println!("Hosting. Join code: {}. Public address: {}", join_code, public_addr);

        public_addr
    } else {
        let addr = socket.local_addr().unwrap();

        socket.send_to(format!("REGISTER {}", join_code).as_bytes(), rendezvous_addr)
            .expect("Failed to register with rendezvous");

        println!("Hosting. Join code: {}. Local address: {}", join_code, addr);

        addr
    };

    let mut buf = [0u8; 512];
    let start_time = Instant::now();
    let timeout = Duration::from_secs(10);

    loop {
        if let Ok((len, _)) = socket.recv_from(&mut buf) {
            let msg = &buf[..len];
            if msg == b"READY" {
                let server_addr = SocketAddr::new(addr.ip(), 6000);
                println!("Server is ready at {}, starting NAT punch", server_addr);
                return server_addr;
            }
        }

        if start_time.elapsed() > timeout {
            panic!("Timeout waiting for READY from rendezvous server");
        }

        sleep(Duration::from_millis(10));
    }
}

/// Binds a UDP socket and sends a join request to the rendezvous server using the join_code
/// Listens for the server address from the rendezvous server and returns it for connection
/// 
/// ConnectionMode::REMOTE is not correctly implemented; the choice was given up midway through development
/// but this code snippet was never removed
/// 
/// Currently join_code is always: ABC123
pub fn join(mode: ConnectionMode, join_code: &str, override_addr: Option<SocketAddr>) -> SocketAddr {
    let rendezvous_addr: SocketAddr = mode.rendezvous_addr(override_addr);

    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Failed to bind client socket");
    socket.set_nonblocking(true).unwrap();

    let _addr = if mode.use_stun() {
        let public_addr = stun_request::get_public_addr(&socket, "stun.l.google.com", 19302)
            .expect("Failed to get public address");

        socket.send_to(format!("JOIN {} {}", join_code, public_addr).as_bytes(), rendezvous_addr)
            .expect("Failed to send join request");

        public_addr
    } else {
        let addr = socket.local_addr().unwrap();

        socket.send_to(format!("JOIN {}", join_code).as_bytes(), rendezvous_addr)
            .expect("Failed to send join request");

        addr
    };

    let mut buf = [0u8; 512];
    loop {
        if let Ok((len, _)) = socket.recv_from(&mut buf) {
            let server_addr_str = String::from_utf8_lossy(&buf[..len]).trim().to_string();
            let server_addr: SocketAddr = server_addr_str.parse()
                .expect("Failed to parse server address");
            println!("Joining server at {}", server_addr);
            return server_addr;
        }
        sleep(Duration::from_millis(10));
    }
}
