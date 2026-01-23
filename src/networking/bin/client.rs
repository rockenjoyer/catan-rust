use catan_rust::networking::{bootstrap, client};
use bevy_quinnet::client::{QuinnetClient, QuinnetClientPlugin, ClientConnectionConfiguration};
use bevy_quinnet::client::connection::ClientAddrConfiguration;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;

fn main() {
    let join_code = std::env::args().nth(1).expect("join code");
    let (_, server_addr) = bootstrap::join("127.0.0.1:4000", &join_code);

    let server_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let mut app = App::new();
    app.add_plugins(QuinnetClientPlugin::default());

    let mut client = app.world_mut().resource_mut::<QuinnetClient>();
    let _ = client.open_connection(ClientConnectionConfiguration {
        addr_config: ClientAddrConfiguration::from_ips(
            server_ip,
            server_addr.port(),
            "0.0.0.0".parse::<IpAddr>().unwrap(),
            0,
        ),
        cert_mode: CertificateVerificationMode::SkipVerification,
        defaultables: Default::default(),
    });

    client::run_client(&mut *client);
}

