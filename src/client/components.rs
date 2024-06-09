use bevy::ecs::component::Component;
use renet::ClientId;

#[derive(Component)]
pub struct PlayerEntity(pub ClientId);

#[derive(Component)]
pub struct MyPlayer;
