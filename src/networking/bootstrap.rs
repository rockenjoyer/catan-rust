use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::thread::sleep;

pub fn host(rendezvous: &str, join_code: &str) -> SocketAddr {
    let rendezvous_addr: SocketAddr = rendezvous.parse()
        .expect("Rendezvous must be a valid SocketAddr, e.g., 127.0.0.1:4000");

    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Failed to bind host socket");
    socket.set_nonblocking(true).unwrap();

    socket.send_to(format!("REGISTER {}", join_code).as_bytes(), rendezvous_addr)
        .expect("Failed to register with rendezvous");

    println!("Hosting. Join code: {}", join_code);

    let mut buf = [0u8; 256];
    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    loop {
        if let Ok((len, _)) = socket.recv_from(&mut buf) {
            let msg = &buf[..len];
            if msg == b"READY" {
                let server_addr = SocketAddr::new(socket.local_addr().unwrap().ip(), 6000);
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

pub fn join(rendezvous: &str, join_code: &str) -> SocketAddr {
    let rendezvous_addr: SocketAddr = rendezvous.parse()
        .expect("Rendezvous must be a valid SocketAddr, e.g., 127.0.0.1:4000");

    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Failed to bind client socket");
    socket.set_nonblocking(true).unwrap();

    socket.send_to(format!("JOIN {}", join_code).as_bytes(), rendezvous_addr)
        .expect("Failed to send join request");

    let mut buf = [0u8; 256];
    loop {
        if let Ok((len, _)) = socket.recv_from(&mut buf) {
            let server_addr_str = String::from_utf8_lossy(&buf[..len]).trim().to_string();
            let addr: SocketAddr = server_addr_str.parse()
                .expect("Failed to parse server address");
            println!("Joining server at {}", addr);
            return addr;
        }
        sleep(Duration::from_millis(10));
    }
}
