use std::{net::{SocketAddrV4, UdpSocket}, time::SystemTime};

use bevy::{app::{App, Startup, Update}, ecs::{entity::Entity, system::Resource}, log::info, utils::HashMap, DefaultPlugins};
use bevy_renet::{transport::{NetcodeClientPlugin, NetcodeServerPlugin}, RenetClientPlugin};
use renet::{transport::{ClientAuthentication, NetcodeClientTransport}, ClientId, ConnectionConfig, RenetClient};

use crate::systems::{handle_keyboard_input_system, handle_lobby_sync_event_system, handle_player_spawn_event_system, receive_message_system, send_message_system, setup_system, update_player_movement_system};

mod systems;
mod events;
mod components;

#[derive(Resource)]
pub struct MyClientId(pub ClientId);

#[derive(Resource)]
pub struct PlayerEntities(pub HashMap<ClientId, Entity>);

fn main() {
    let mut app = App::new();
    app.add_plugins(RenetClientPlugin);

    let client = RenetClient::new(ConnectionConfig::default());
    app.insert_resource(client);

    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);
    app.add_plugins(DefaultPlugins);

    let client_id = rand::random::<u64>();
    app.insert_resource(MyClientId(ClientId::from_raw(client_id)));
    app.insert_resource(PlayerEntities(HashMap::new()));

    let authentication = ClientAuthentication::Unsecure {
        server_addr: std::net::SocketAddr::V4(SocketAddrV4::new(
            std::net::Ipv4Addr::new(127, 0, 0, 1),
            5000,
        )),
        client_id,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(transport);

    app.add_systems(Update, send_message_system);
    app.add_systems(Update, receive_message_system);
    app.add_systems(Update, handle_keyboard_input_system);
    app.add_systems(Update, handle_player_spawn_event_system);
    app.add_systems(Update, update_player_movement_system);
    app.add_systems(Update, handle_lobby_sync_event_system);
    app.add_systems(Startup, setup_system);

    info!("Starting client {}", client_id);

    app.run();
}
