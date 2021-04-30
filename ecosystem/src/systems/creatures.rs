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
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Creature>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    let offset = 5.0;

    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        let minx = -hw + collider.size.x + offset;
        let maxx = hw - collider.size.x - offset;
        let miny = -hh + collider.size.y + offset;
        let maxy = hh - collider.size.y - offset;

        //rigidbody.wrap(&mut transform, minx, maxx, miny, maxy);
        //rigidbody.bounce(&mut transform, minx, maxx, miny, maxy);
        rigidbody.contain(&mut transform, minx, maxx, miny, maxy);
    }
}

/// Fly behavior
pub fn fly_update(mut query: Query<&mut Fly>) {
    for mut _fly in query.iter_mut() {}
}

/// Fly behavior
pub fn fly_physics(mut random: ResMut<Random>, mut query: Query<&mut Rigidbody, With<Fly>>) {
    for mut rigidbody in query.iter_mut() {
        let direction = random.direction();
        let modifier = random.random();
        rigidbody.apply_force(direction * (FLY_FORCE * modifier as f32), "fly");
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
        let modifier = noise.get(t, 0.5);
        rigidbody.apply_force(direction * FISH_FORCE * modifier as f32, "swim");
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
        let modifier = noise.get(t, 0.5);
        rigidbody.apply_force(direction * SNAKE_GROUND_FORCE * modifier as f32, "slither");
    }
}
