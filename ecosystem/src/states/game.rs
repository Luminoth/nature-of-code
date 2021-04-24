//! Game state systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;

use crate::components::camera::CameraOrtho2dBundle;

use crate::CAMERA_SIZE;

/// Game setup
pub fn setup(mut commands: Commands) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(CameraOrtho2dBundle::new(CAMERA_SIZE));

    // physics
    commands.insert_resource(RapierConfiguration::default());

    // TODO: game state

    // TODO: world
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<RapierConfiguration>();
    commands.remove_resource::<ClearColor>();
}
