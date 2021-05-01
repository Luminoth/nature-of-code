//! Creature systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::resources::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CreaturesSystem {
    Update,
    Physics,
}

/// Fly behavior
pub fn fly_update(mut query: Query<&mut Fly>) {
    for mut _fly in query.iter_mut() {}
}

/// Fly behavior
pub fn fly_physics(mut random: ResMut<Random>, mut query: Query<&mut Rigidbody, With<Fly>>) {
    for mut rigidbody in query.iter_mut() {
        let direction = random.direction();
        let modifier = random.random() as f32;
        let magnitude = FLY_ACCEL * rigidbody.mass * modifier;
        rigidbody.apply_force(direction * magnitude, "fly");
    }
}

/// Fish behavior
pub fn fish_update(mut query: Query<&mut Fish>) {
    for mut _fish in query.iter_mut() {}
}

/// Fish behavior
pub fn fish_physics(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<&mut Rigidbody, With<Fish>>,
) {
    for mut rigidbody in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);

        let direction = random.direction();
        let modifier = noise.get(t, 0.5) as f32;
        let magnitude = FISH_ACCEL * rigidbody.mass * modifier;
        rigidbody.apply_force(direction * magnitude, "swim");
    }
}

/// Snake behavior
pub fn snake_update(mut query: Query<&mut Snake>) {
    for mut _snake in query.iter_mut() {}
}

/// Snake behavior
pub fn snake_physics(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<&mut Rigidbody, With<Snake>>,
) {
    for mut rigidbody in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);

        let direction = random.direction();
        let modifier = noise.get(t, 0.5) as f32;
        let magnitude = SNAKE_GROUND_ACCEL * rigidbody.mass * modifier;
        rigidbody.apply_force(direction * magnitude, "slither");
    }
}
