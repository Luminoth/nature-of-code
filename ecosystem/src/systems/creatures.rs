//! Creature systems

use bevy::prelude::*;

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
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut physics) in query.iter_mut() {
        physics.acceleration = vec2_uniform() * noise.get_perlin() as f32 * 0.5;
        physics.update(&mut transform);
        Physics::wrap(&mut transform, -hw, hw, -hh, hh);
    }
}

/// Fish behavior
pub fn fish(
    noise: Res<Noise>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Physics), With<Fish>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut physics) in query.iter_mut() {
        physics.acceleration = vec2_uniform() * noise.get_perlin() as f32 * 1.5;
        physics.update(&mut transform);
        Physics::wrap(&mut transform, -hw, hw, -hh, hh);
    }
}

/// Snake behavior
pub fn snake(
    noise: Res<Noise>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Physics), With<Snake>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut physics) in query.iter_mut() {
        physics.acceleration = vec2_uniform() * noise.get_perlin() as f32 * 1.0;
        physics.update(&mut transform);
        Physics::wrap(&mut transform, -hw, hw, -hh, hh);
    }
}
