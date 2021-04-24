//! Creature systems

use bevy::prelude::*;
use rand::Rng;

use crate::components::creatures::*;
use crate::components::*;
use crate::resources::*;
use crate::vec2_uniform;

/// Fly behavior
pub fn fly(
    noise: Res<Noise>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Physics), With<Fly>>,
) {
    let mut rng = rand::thread_rng();

    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut physics) in query.iter_mut() {
        physics.acceleration = vec2_uniform() * noise.get_perlin(rng.gen_range(0.0..0.5)) as f32;
        physics.update(&mut transform);
        Physics::wrap(&mut transform, -hw, hw, -hh, hh);
    }
}

/// Fish behavior
pub fn fish(mut query: Query<(&mut Transform, &mut Physics), With<Fish>>) {
    for (mut transform, mut physics) in query.iter_mut() {
        physics.update(&mut transform);
    }
}

/// Snake behavior
pub fn snake(mut query: Query<(&mut Transform, &mut Physics), With<Snake>>) {
    for (mut transform, mut physics) in query.iter_mut() {
        physics.update(&mut transform);
    }
}
