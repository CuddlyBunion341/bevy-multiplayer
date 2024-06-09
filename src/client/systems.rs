use bevy::{
    asset::Assets,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::Without,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    math::primitives::Cuboid,
    pbr::{MaterialMeshBundle, StandardMaterial},
    prelude::default,
    render::mesh::Mesh,
    transform::components::Transform,
    utils::{warn, HashMap},
};
use multiplayer_demo::PlayerSync;
use renet::{ClientId, DefaultChannel, RenetClient};

use crate::{
    components::{MyPlayer, PlayerEntity},
    events::{LobbySyncEvent, PlayerDespawnEvent, PlayerSpawnEvent},
    MyClientId,
};

pub fn send_message_system() {}

pub fn receive_message_system(
    mut client: ResMut<RenetClient>,
    mut spawn_events: EventWriter<PlayerSpawnEvent>,
    mut despawn_events: EventWriter<PlayerDespawnEvent>,
    mut lobby_sync_events: EventWriter<LobbySyncEvent>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            multiplayer_demo::RenetEvent::PlayerJoin(client_id) => {
                info!("Client connected: {}", client_id);
                spawn_events.send(PlayerSpawnEvent(client_id));
            }
            multiplayer_demo::RenetEvent::PlayerLeave(client_id) => {
                info!("Client disconnected: {}", client_id);
                despawn_events.send(PlayerDespawnEvent(client_id));
            }
            _ => {}
        }
    }

    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let message = bincode::deserialize(&message).unwrap();

        match message {
            multiplayer_demo::RenetEvent::LobbySync(positions) => {
                lobby_sync_events.send(LobbySyncEvent(positions));
            }
            _ => {}
        }
    }
}

pub fn handle_keyboard_input_system() {}

pub fn update_player_movement_system() {}

pub fn setup_system(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

pub fn handle_player_spawn_event_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_events: EventReader<PlayerSpawnEvent>,
) {
    for event in spawn_events.read() {
        let client_id = event.0;

        commands.spawn((
            MaterialMeshBundle {
                material: materials.add(StandardMaterial::default()),
                mesh: meshes.add(Cuboid::default()),

                ..default()
            },
            PlayerEntity(client_id),
        ));
    }
}

pub fn handle_lobby_sync_event_system(
    mut sync_events: EventReader<LobbySyncEvent>,
    mut query: Query<(&PlayerEntity, &mut Transform), Without<MyPlayer>>,
) {
    let event_option = sync_events.read().last();

    if event_option.is_none() {
        return;
    }

    let event = event_option.unwrap();

    for (player_entity, mut transform) in query.iter_mut() {
        for (client_id, player_sync) in event.0.iter() {
            if *client_id == player_entity.0 {
                let new_position = player_sync.position;
                transform.translation = new_position.into();
            }
        }
    }
}
