use bevy::{ecs::system::{Res, ResMut, Resource}, log::info, utils::HashMap};
use renet::{ClientId, DefaultChannel, RenetServer};
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

pub fn receive_message_system() {
}
pub fn handle_events_system() {}
