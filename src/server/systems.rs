use std::collections::HashMap;

use bevy::{ecs::{event::EventReader, system::{Res, ResMut, Resource}}, log::info};
use multiplayer_demo::PlayerSync;
use renet::{ClientId, DefaultChannel, RenetServer, ServerEvent};
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct PlayerLobby(pub HashMap<ClientId, PlayerSync>);

use crate::SERVER_ADDR;

pub fn setup_system() {
    info!("Server started on {}", SERVER_ADDR);
}

pub fn send_message_system(mut server: ResMut<RenetServer>, player_lobby: Res<PlayerLobby>) {
    let chanel = DefaultChannel::Unreliable;
    let lobby = player_lobby.0.clone();
    let event = multiplayer_demo::RenetEvent::LobbySync(lobby);
    let message = bincode::serialize(&event).unwrap();
    print_lobby(&player_lobby);
    server.broadcast_message(chanel, message);
}

fn print_lobby(lobby: &PlayerLobby) {
    info!("Lobby:");
    info!("------");

    if lobby.0.is_empty() {
        info!("Empty");
        return;
    }

    for (client_id, player) in lobby.0.iter() {
        info!("Client {}: {:?}", client_id, player);
    }
}

pub fn receive_message_system(mut server: ResMut<RenetServer>, mut player_lobby: ResMut<PlayerLobby>) {
    for client_id in server.clients_id() {
        let message = server.receive_message(client_id, DefaultChannel::Unreliable);
        if let Some(message) = message {
            let player: PlayerSync = bincode::deserialize(&message).unwrap();
            player_lobby.0.insert(client_id, player);
        }
    }
}

pub fn handle_events_system(mut server: ResMut<RenetServer>, mut server_events: EventReader<ServerEvent>, mut player_lobby: ResMut<PlayerLobby>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
                player_lobby.0.insert(*client_id, PlayerSync { position: [0.0, 0.0, 0.0] });
                server.broadcast_message_except(*client_id, DefaultChannel::ReliableOrdered, bincode::serialize(&multiplayer_demo::RenetEvent::PlayerJoin(*client_id)).unwrap());
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
                player_lobby.0.remove(client_id);
                server.broadcast_message(DefaultChannel::ReliableOrdered, bincode::serialize(&multiplayer_demo::RenetEvent::PlayerLeave(*client_id)).unwrap());
            }
        }
    }
}

