//! Creature systems

use bevy::prelude::*;
use noise::Perlin;

use crate::components::creatures::*;
use crate::components::*;

/// Fly behavior
pub fn fly(_noise: Res<Perlin>, mut query: Query<(&mut Transform, &mut Physics), With<Fly>>) {
    for (mut transform, mut physics) in query.iter_mut() {
        let acceleration = physics.acceleration;

        physics.velocity += acceleration;
        transform.translation += physics.velocity;
    }
}

/// Fish behavior
pub fn fish(mut query: Query<(&mut Transform, &mut Physics), With<Fish>>) {
    for (mut transform, mut physics) in query.iter_mut() {
        let acceleration = physics.acceleration;

        physics.velocity += acceleration;
        transform.translation += physics.velocity;
    }
}

/// Snake behavior
pub fn snake(mut query: Query<(&mut Transform, &mut Physics), With<Snake>>) {
    for (mut transform, mut physics) in query.iter_mut() {
        let acceleration = physics.acceleration;

        physics.velocity += acceleration;
        transform.translation += physics.velocity;
    }
}
