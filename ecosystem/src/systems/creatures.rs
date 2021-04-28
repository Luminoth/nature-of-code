//! Creature systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::resources::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CreaturesSystem {
    Creature,
    Physics,
}

/// Common creature behavior
pub fn creature_physics(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Rigidbody), With<Creature>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut rigidbody) in query.iter_mut() {
        //rigidbody.wrap(&mut transform, -hw, hw, -hh, hh);
        //rigidbody.bounce(&mut transform, -hw, hw, -hh, hh);
        rigidbody.contain(&mut transform, -hw, hw, -hh, hh);
    }
}

/// Fly behavior
pub fn fly_update(mut query: Query<&Fly>) {
    for mut _fly in query.iter_mut() {}
}

/// Fly behavior
pub fn fly_physics(
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<&mut Rigidbody, With<Fly>>,
) {
    for mut rigidbody in query.iter_mut() {
        let direction = random.direction();
        //let direction = noise.direction(&mut random, 0.5);
        rigidbody.apply_force(
            direction * (FLY_FORCE * noise.sample(&mut random, 0.5) as f32),
            "fly",
        );
    }
}

/// Fish behavior
pub fn fish_update(
    time: Res<Time>,
    mut random: ResMut<Random>,
    _noise: Res<PerlinNoise>,
    mut query: Query<&mut Fish>,
) {
    for mut fish in query.iter_mut() {
        if fish.swim_timer.tick(time.delta()).just_finished() {
            fish.swim_cooldown.reset();
        }

        if fish.swim_cooldown.tick(time.delta()).just_finished() {
            fish.swim_direction = random.direction();
            //fish.swim_direction = _noise.direction(&mut random, 0.5);

            fish.swim_timer.reset();
        }
    }
}

/// Fish behavior
pub fn fish_physics(
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Rigidbody, &Fish)>,
) {
    for (mut rigidbody, fish) in query.iter_mut() {
        if !fish.swim_timer.finished() {
            rigidbody.apply_force(
                fish.swim_direction * FISH_FORCE * noise.sample(&mut random, 0.5) as f32,
                "swim",
            );
        }
    }
}

/// Snake behavior
pub fn snake_update(
    time: Res<Time>,
    mut random: ResMut<Random>,
    _noise: Res<PerlinNoise>,
    mut query: Query<&mut Snake>,
) {
    for mut snake in query.iter_mut() {
        if snake.direction_timer.tick(time.delta()).just_finished() {
            snake.direction = random.direction();
            //snake.direction = _noise.direction(&mut random, 0.5);
        }
    }
}

/// Snake behavior
pub fn snake_physics(
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&mut Rigidbody, &Snake)>,
) {
    for (mut rigidbody, snake) in query.iter_mut() {
        rigidbody.apply_force(
            snake.direction * SNAKE_GROUND_FORCE * noise.sample(&mut random, 0.5) as f32,
            "slither",
        );
    }
}
