use bytes::Bytes;
use bincode;

use bevy_quinnet::{
    server::{
        endpoint::Endpoint, QuinnetServer, ServerReceiveError
    },
    client::{
        ConnectionClosed, QuinnetClient, connection::ClientSideConnection
    },
    shared::{
        ClientId, channels::ChannelId
    }
};

use crate::backend::networking::protocol::{ServerMessage, ClientMessage};


pub struct ServerTransport {
    server: QuinnetServer,
}

// send_payload_on => todo: implement manual channel-config

impl ServerTransport {
    pub fn new(server: QuinnetServer) -> Self {
        ServerTransport { server }
    }

    pub fn send(&mut self, client_id: ClientId, channel_id: ChannelId, msg: &ServerMessage) {
        let payload = bincode::serialize(msg).unwrap();
        let endpoint: &mut Endpoint = self.server.endpoint_mut();
        endpoint.send_payload(client_id, payload);
    }

    pub fn recv(&mut self, client_id: ClientId, channel_id: ChannelId) {
        let endpoint: &mut Endpoint = self.server.endpoint_mut();
        endpoint.receive_payload(client_id, channel_id);

    }
    /*
    pub fn update(&mut self, delta: Duration) {
        self.server.update(delta);
    }
    */
    /*
    pub fn get_client_address(&self, client_id: ClientId) -> std::net::SocketAddr {
        self.server.client_address(client_id)
    }
    */
}

pub struct ClientTransport {
    client: QuinnetClient,
}

impl ClientTransport {
    pub fn new(client: QuinnetClient) -> Self {
        ClientTransport { client }
    }

    pub fn connection(&mut self) -> &mut ClientSideConnection {
        self.client.connection_mut()
    }

    pub fn send(&mut self, channel_id: ChannelId, msg: &ClientMessage) {
        let payload = bincode::serialize(msg).unwrap();
        let connection = self.client.connection_mut();
        connection.send_payload(payload);
    }

    pub fn recv(&mut self) -> Result<Option<Bytes>, ConnectionClosed> {
        self.client.connection_mut().receive_payload(1) // channel_id hardcoded for now to 1 as default
    }
    /*
    pub fn update(&mut self, delta: Duration) {
        self.client.update(delta);
    }
    */
}
