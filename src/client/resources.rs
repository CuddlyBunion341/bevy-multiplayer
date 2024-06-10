use std::collections::HashMap;

use bevy::ecs::{entity::Entity, system::Resource};
use renet::ClientId;

#[derive(Resource)]
pub struct MyClientId(pub ClientId);

#[derive(Resource)]
pub struct PlayerEntities(pub HashMap<ClientId, Entity>);
