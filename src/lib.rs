use std::collections::HashMap;

use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSync {
    pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RenetEvent {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    LobbySync(HashMap<ClientId, PlayerSync>)
}
