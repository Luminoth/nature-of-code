//! Creature systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::resources::*;
use crate::vec2_uniform;

/// Common creature behavior
pub fn creature_after(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Rigidbody), With<Creature>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut rigidbody) in query.iter_mut() {
        //rigidbody.wrap(&mut transform, -hw, hw, -hh, hh);
        rigidbody.bounce(&mut transform, -hw, hw, -hh, hh);
    }
}

/// Fly behavior
pub fn fly(
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<&mut Rigidbody, With<Fly>>,
) {
    for mut rigidbody in query.iter_mut() {
        rigidbody.apply_force(
            vec2_uniform(&mut *random) * noise.sample(&mut *random, 0.5) as f32 * 1000.0,
        );
    }
}

/// Fish behavior
pub fn fish(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Rigidbody, &mut Fish)>,
) {
    for (mut rigidbody, mut fish) in query.iter_mut() {
        if fish.timer.tick(time.delta()).just_finished() {
            rigidbody.apply_force(
                vec2_uniform(&mut *random) * noise.sample(&mut *random, 1.0) as f32 * 5000.0,
            );
        }
    }
}

/// Snake behavior
pub fn snake(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Rigidbody, &mut Snake)>,
) {
    for (mut rigidbody, mut snake) in query.iter_mut() {
        if snake.timer.tick(time.delta()).just_finished() {
            rigidbody.apply_force(
                vec2_uniform(&mut *random) * noise.sample(&mut *random, 1.0) as f32 * 10000.0,
            );
        }
    }
}
