//! Game state systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::rapier::math::Vector;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::components::*;
use crate::resources::*;

/// Game setup
pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut random: ResMut<Random>,
    simulation: Res<SimulationParams>,
    world_bounds: Res<WorldBounds>,
) {
    // cameras
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = world_bounds.height / 2.0;

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    // physics
    rapier_config.gravity = Vector::zeros();

    // world bounds
    let qw = world_bounds.width / 4.0;
    let hw = world_bounds.width / 2.0;
    let hh = world_bounds.height / 2.0;

    // environment

    // ground
    Ground::spawn(
        &mut commands,
        0,
        Vec2::new(hw - qw / 2.0, 0.0),
        Vec2::new(qw, world_bounds.height),
    );

    // water
    Water::spawn(
        &mut commands,
        0,
        Vec2::new(-hw + (qw * 3.0) / 2.0, 0.0),
        Vec2::new(qw * 3.0, world_bounds.height),
    );

    // air
    Air::spawn(
        &mut commands,
        0,
        Vec2::new(world_bounds.width, world_bounds.height),
    );

    // creatures

    // flies
    for i in 0..simulation.fly_count {
        let position =
            random.vec2_range(-hw + FLY_SIZE..hw - FLY_SIZE, -hh + FLY_SIZE..hh - FLY_SIZE);
        Fly::spawn(&mut commands, &mut random, i, position, FIREFLY_COLOR);
    }

    // fish
    for i in 0..simulation.fish_count {
        let position = random.vec2_range(
            -hw + FISH_WIDTH..qw - FISH_WIDTH,
            -hh + FISH_LENGTH..hh - FISH_LENGTH,
        );
        Fish::spawn(&mut commands, &mut random, i, position, Color::NAVY);
    }

    // snakes
    for i in 0..simulation.snake_count {
        let position = random.vec2_range(
            qw + SNAKE_WIDTH..hw - SNAKE_WIDTH,
            -hh + SNAKE_LENGTH..hh - SNAKE_LENGTH,
        );
        Snake::spawn(&mut commands, &mut random, i, position);
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
