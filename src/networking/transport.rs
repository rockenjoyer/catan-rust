use crossbeam_channel::{Receiver, Sender};
use crate::networking::protocol::{ClientMessage, ServerMessage};

// receives messages from clients, sends messages to clients
pub struct ServerTransport {
    pub incoming: Receiver<ClientMessage>,
    pub outgoing: Sender<ServerMessage>,
}

impl ServerTransport {
    pub fn new(
        incoming: Receiver<ClientMessage>,
        outgoing: Sender<ServerMessage>,
    ) -> Self {
        Self { incoming, outgoing }
    }
}

// receives messages from server, sends messages to server
pub struct ClientTransport {
    pub incoming: Receiver<ServerMessage>,
    pub outgoing: Sender<ClientMessage>,
}

impl ClientTransport {
    pub fn new(
        incoming: Receiver<ServerMessage>,
        outgoing: Sender<ClientMessage>,
    ) -> Self {
        Self { incoming, outgoing }
    }
}

pub fn create_server() -> ServerTransport {
    let (_in_tx, in_rx) = crossbeam_channel::unbounded::<ClientMessage>();
    let (out_tx, _out_rx) = crossbeam_channel::unbounded::<ServerMessage>();

    ServerTransport::new(in_rx, out_tx)
}

pub fn create_client(_addr: &str) -> ClientTransport {
    let (_in_tx, in_rx) = crossbeam_channel::unbounded::<ServerMessage>();
    let (out_tx, _out_rx) = crossbeam_channel::unbounded::<ClientMessage>();

    ClientTransport::new(in_rx, out_tx)
}
