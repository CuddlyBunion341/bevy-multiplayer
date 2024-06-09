use std::collections::HashMap;

use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerAttributes {
    pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    PlayerMove([f32; 3])
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    LobbySync(HashMap<ClientId, PlayerAttributes>),
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
}
