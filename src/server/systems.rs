use bevy::{ecs::{event::EventReader, system::{Res, ResMut, Resource}}, log::info, utils::HashMap};
use renet::{ClientId, DefaultChannel, RenetServer, ServerEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    position: [f32; 3],
}

#[derive(Resource)]
pub struct PlayerLobby(pub HashMap<ClientId, Player>);

use crate::SERVER_ADDR;

pub fn setup_system() {
    info!("Server started on {}", SERVER_ADDR);
}

pub fn send_message_system(mut server: ResMut<RenetServer>, player_lobby: Res<PlayerLobby>) {
    let chanel = DefaultChannel::Unreliable;
    let message = bincode::serialize(&player_lobby.0).unwrap();
    server.broadcast_message(chanel, message);
}

pub fn receive_message_system(mut server: ResMut<RenetServer>, mut player_lobby: ResMut<PlayerLobby>) {
    for client_id in server.clients_id() {
        let message = server.receive_message(client_id, DefaultChannel::Unreliable);
        if let Some(message) = message {
            let player: Player = bincode::deserialize(&message).unwrap();
            player_lobby.0.insert(client_id, player);
        }
    }
}

pub fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}

