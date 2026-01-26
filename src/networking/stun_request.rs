use std::net::{SocketAddr, UdpSocket, ToSocketAddrs};
use std::time::Duration;

use stun::message::{Message, BINDING_REQUEST, Getter};
use stun::xoraddr::XorMappedAddress;

pub fn get_public_addr(socket: &UdpSocket, stun_host: &str, stun_port: u16) -> std::io::Result<SocketAddr> {
    socket.set_read_timeout(Some(Duration::from_secs(3)))?;

    let stun_server = resolve_ipv4(stun_host, stun_port)?;

    let mut request = Message::new();
    request.set_type(BINDING_REQUEST);
    request.new_transaction_id();
    request.encode();

    socket.send_to(&request.raw, stun_server)?;

    let mut buf = [0u8; 1024];
    let (len, _) = socket.recv_from(&mut buf)?;

    let mut response = Message::new();
    response.raw = buf[..len].to_vec();
    response.decode();

    let mut xma = XorMappedAddress::default();
    xma.get_from(&response);

    Ok(SocketAddr::new(xma.ip, xma.port))
}

fn resolve_ipv4(host: &str, port: u16) -> std::io::Result<SocketAddr> {
    (host, port)
        .to_socket_addrs()?
        .find(|a| a.is_ipv4())
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("No IPv4 address found for {}", host)
        ))
}
