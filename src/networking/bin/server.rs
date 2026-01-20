use catan_rust::networking::{bootstrap, server};
use bevy_quinnet::server::{QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration, EndpointAddrConfiguration};
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use std::net::{Ipv4Addr, Ipv6Addr};
use bevy::prelude::*;
use rand::distr::Alphanumeric;
use rand::Rng;

use catan_rust::networking::server::run_server;

fn main() {
    /*let join_code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    */
    let join_code = "ABC123".to_string();
    println!("Server started. Join code: {}", join_code);

    let (socket, server_addr) = bootstrap::host("127.0.0.1:4000", &join_code);
    println!("Bootstrap server running at {}", server_addr);

    let mut app = App::new();
    app.add_plugins(QuinnetServerPlugin::default());

    let mut server = app.world_mut().get_resource_mut::<QuinnetServer>().unwrap();
    let _ = server.start_endpoint(
        ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_ip(Ipv4Addr::UNSPECIFIED, 6000),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: Ipv4Addr::LOCALHOST.to_string(),
            },
            defaultables: Default::default(),
        }
    );
    println!("Endpoint started");

    let endpoint = server.endpoint_mut();
    run_server(endpoint);
}
