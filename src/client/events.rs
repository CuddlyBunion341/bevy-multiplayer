use bevy::{ecs::event::Event, utils::HashMap};
use multiplayer_demo::PlayerSync;
use renet::ClientId;

#[derive(Event)]
pub struct PlayerSpawnEvent(pub ClientId);

#[derive(Event)]
pub struct PlayerDespawnEvent(pub ClientId);

#[derive(Event)]
pub struct PlayerMoveEvent(pub ClientId, pub [f32; 3]);

#[derive(Event)]
pub struct LobbySyncEvent(pub HashMap<ClientId, PlayerSync>);
