use bevy::render::camera::SortedCamera;
use bevy_quinnet::server::endpoint::Endpoint;
use bevy_quinnet::shared::ClientId;
use bevy_quinnet::shared::channels::ChannelId;
use bevy_quinnet::server::{QuinnetServer, ServerReceiveError};
use bevy_quinnet::client::{ConnectionClosed, QuinnetClient, connection};
use bytes::Bytes;
use bevy_quinnet::client::connection::ClientSideConnection;

use crate::networking_test::protocol::*;

use std::time::Duration;
use bincode;

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
