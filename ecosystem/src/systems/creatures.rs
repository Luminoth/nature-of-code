//! Creature systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::*;
use crate::resources::*;
use crate::vec2_uniform;

/// Common creature behavior
pub fn creature_after(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Physics), With<Creature>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut physics) in query.iter_mut() {
        //physics.wrap(&mut transform, -hw, hw, -hh, hh);
        physics.bounce(&mut transform, -hw, hw, -hh, hh);
    }
}

/// Fly behavior
pub fn fly(
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Transform, &mut Physics), With<Fly>>,
) {
    for (mut transform, mut physics) in query.iter_mut() {
        physics.acceleration = Vec3::from((
            vec2_uniform(&mut *random) * noise.sample(&mut *random, 0.5) as f32,
            0.0,
        ));
        physics.update(&mut transform);
    }
}

/// Fish behavior
pub fn fish(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Transform, &mut Physics, &mut Fish)>,
) {
    for (mut transform, mut physics, mut fish) in query.iter_mut() {
        if fish.timer.tick(time.delta()).just_finished() {
            physics.acceleration = Vec3::from((
                vec2_uniform(&mut *random) * noise.sample(&mut *random, 1.5) as f32,
                0.0,
            ));
        }

        physics.update(&mut transform);
    }
}

/// Snake behavior
pub fn snake(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Transform, &mut Physics, &mut Snake)>,
) {
    for (mut transform, mut physics, mut snake) in query.iter_mut() {
        if snake.timer.tick(time.delta()).just_finished() {
            physics.acceleration = Vec3::from((
                vec2_uniform(&mut *random) * noise.sample(&mut *random, 1.5) as f32,
                0.0,
            ));
        }

        physics.update(&mut transform);
    }
}
