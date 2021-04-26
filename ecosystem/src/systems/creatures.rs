//! Creature systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::resources::*;

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
    _noise: Res<PerlinNoise>,
    mut query: Query<&mut Rigidbody, With<Fly>>,
) {
    for mut rigidbody in query.iter_mut() {
        rigidbody.apply_force(
            random.direction() * 0.01, /*noise.sample(&mut *random, 1000.0) as f32*/
        );
    }
}

/// Fish behavior
pub fn fish(
    time: Res<Time>,
    mut random: ResMut<Random>,
    _noise: Res<PerlinNoise>,
    mut query: Query<(&mut Rigidbody, &mut Fish)>,
) {
    for (mut rigidbody, mut fish) in query.iter_mut() {
        if !fish.swim_timer.finished() {
            rigidbody.apply_force(
                fish.swim_direction * 500.0, /*noise.sample(&mut *random, 300.0) as f32*/
            );
        } else if fish.swim_timer.tick(time.delta()).just_finished() {
            fish.swim_cooldown.reset();
        }

        if fish.swim_cooldown.tick(time.delta()).just_finished() {
            fish.swim_direction = random.direction();

            fish.swim_timer.reset();
        }
    }
}

/// Snake behavior
pub fn snake(
    time: Res<Time>,
    mut random: ResMut<Random>,
    _noise: Res<PerlinNoise>,
    mut query: Query<(&mut Rigidbody, &mut Snake)>,
) {
    for (mut rigidbody, mut snake) in query.iter_mut() {
        // TODO: not quite this
        if snake.direction_timer.tick(time.delta()).just_finished() {
            rigidbody.apply_force(
                random.direction() * 1500.0, /*noise.sample(&mut *random, 10000.0) as f32*/
            );
        }
    }
}
