use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{
    app::{App, Startup, Update}, log::LogPlugin, MinimalPlugins
};
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer,
};
use systems::{handle_events_system, receive_message_system, send_message_system, setup_system, PlayerLobby};
mod systems;

const SERVER_ADDR: &str = "127.0.0.1:5000";

fn main() {
    let mut app = App::new();

    // base plugins
    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin::default());
    app.add_plugins(RenetServerPlugin);

    // renet server
    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    app.add_plugins(NetcodeServerPlugin);
    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(transport);

    // game systems
    app.insert_resource(PlayerLobby(HashMap::default()));

    app.add_systems(Startup, setup_system);
    app.add_systems(Update, send_message_system);
    app.add_systems(Update, receive_message_system);
    app.add_systems(Update, handle_events_system);
    app.run();
}
