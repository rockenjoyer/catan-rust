/// WARNING:
/// The code in this file is not actively used. It is implemented in bootstrap.rs to find the public IP of the host machine
/// if ConnectionMode::REMOTE, but this is currently not operational.
/// Since only LAN multiplayer is functional, this file is never called and might not work correctly
/// in the current networking state.

use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};

use stun::message::{Message, BINDING_REQUEST, Getter};
use stun::xoraddr::XorMappedAddress;

/// Sends a STUN binding request to the specified STUN server to discover the public IP and port
/// of the host. Waits for response, decodes it, and extracts the public address using XorMappedAddress
/// 
/// Returns the public address or an error if the request times out or fails
pub fn get_public_addr(socket: &UdpSocket, stun_host: &str, stun_port: u16) -> std::io::Result<SocketAddr> {
    let stun_server = resolve_ipv4(stun_host, stun_port)?;
    println!("STUN server address: {}", stun_server);

    let mut request = Message::new();
    request.set_type(BINDING_REQUEST);
    let _ = request.new_transaction_id().is_ok();
    request.encode();
    println!("Sending STUN request to {}", stun_server);

    socket.send_to(&request.raw, stun_server)?;

    let mut buf = [0u8; 1024];
    let start_time = Instant::now();
    let timeout = Duration::from_secs(3);

    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, _)) => {
                println!("Received {} bytes", len);
                let mut response = Message::new();
                response.raw = buf[..len].to_vec();

                if response.decode().is_ok() {
                    println!("Decoded STUN response successfully");
                    let mut xma = XorMappedAddress::default();

                    if xma.get_from(&response).is_ok() {
                        let public_addr = SocketAddr::new(xma.ip, xma.port);
                        println!("Public address: {}", public_addr);
                        return Ok(public_addr);
                    } else {
                        println!("Failed to get XorMappedAddress from STUN response");
                    }
                } else {
                    println!("Failed to decode STUN response");
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if start_time.elapsed() >= timeout {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "STUN request timed out",
                    ));
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

/// Resolves the given hostname and port to an IPv4 socket address.
/// 
/// Returns the first IPv4 address found or an error if no IPv4 address is available.
fn resolve_ipv4(host: &str, port: u16) -> std::io::Result<SocketAddr> {
    (host, port)
        .to_socket_addrs()?
        .find(|a| a.is_ipv4())
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("No IPv4 address found for {}", host)
        ))
}
