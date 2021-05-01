//! Game state systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::resources::*;

/// Game setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut random: ResMut<Random>,
    windows: Res<Windows>,
) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let window = windows.get_primary().unwrap();
    let qw = window.width() as f32 / 4.0;
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    // environment

    // ground
    Ground::spawn(
        &mut commands,
        Vec2::new(hw - qw / 2.0, 0.0),
        Vec2::new(qw, window.height()),
    );

    // water
    Water::spawn(
        &mut commands,
        Vec2::new(-hw + (qw * 3.0) / 2.0, 0.0),
        Vec2::new(qw * 3.0, window.height()),
    );

    // air
    Air::spawn(&mut commands, Vec2::new(window.width(), window.height()));

    // creatures
    // TODO: read the creature counts from a resource that we supply in main for easier debugging

    // flies
    for id in 0..random.normal_clamped::<f32>(10.0, 3.0, 3.0, 20.0) as u32 {
        let position = random.vec2_range(-hw + 5.0..hw - 5.0, -hh + 5.0..hh - 5.0);
        Fly::spawn(&mut commands, &asset_server, id, position);
    }

    // fish
    for id in 0..random.normal_clamped::<f32>(4.0, 3.0, 2.0, 8.0) as u32 {
        let position = random.vec2_range(-hw + 10.0..qw - 10.0, -hh + 10.0..hh - 10.0);
        Fish::spawn(&mut commands, &asset_server, id, position);
    }

    // snakes
    for id in 0..random.normal_clamped::<f32>(2.0, 1.0, 1.0, 4.0) as u32 {
        let position = random.vec2_range(qw + 5.0..hw - 5.0, -hh + 5.0..hh - 5.0);
        Snake::spawn(&mut commands, &asset_server, id, position);
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
