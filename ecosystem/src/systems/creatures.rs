//! Creature systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::resources::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CreaturesSystem {
    Update,
    Physics,
    Bounds,
}

const BOUNDS_OFFSET: f32 = 5.0;

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

/// Keep flies inside the window
pub fn fly_bounds(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Fly>>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        let (minx, maxx, miny, maxy) =
            collider.adjust_container_bounds(-hw, hw, -hh, hh, BOUNDS_OFFSET);
        rigidbody.contain(&mut transform, minx, maxx, miny, maxy);
        rigidbody.repel(&mut transform, minx, maxx, miny, maxy);
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

/// Keep fish inside the water
pub fn fish_bounds(
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Fish>>,
    fluids: Query<(&Transform, &Collider), (With<Fluid>, Without<Fish>)>,
) {
    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        for (ftransform, fcollider) in fluids.iter() {
            if collider.collides(&transform, fcollider, ftransform) {
                let hw = fcollider.size.x / 2.0;
                let hh = fcollider.size.y / 2.0;

                let minx = ftransform.translation.x - hw;
                let maxx = ftransform.translation.x + hw;
                let miny = ftransform.translation.y - hh;
                let maxy = ftransform.translation.y + hh;

                let (minx, maxx, miny, maxy) =
                    collider.adjust_container_bounds(minx, maxx, miny, maxy, BOUNDS_OFFSET);
                rigidbody.contain(&mut transform, minx, maxx, miny, maxy);
                rigidbody.repel(&mut transform, minx, maxx, miny, maxy);
            }
        }
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

/// Keep snakes on the ground (for now)
pub fn snake_bounds(
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Snake>>,
    surfaces: Query<(&Transform, &Collider), (With<Surface>, Without<Snake>)>,
) {
    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        for (stransform, scollider) in surfaces.iter() {
            if collider.collides(&transform, scollider, stransform) {
                let hw = scollider.size.x / 2.0;
                let hh = scollider.size.y / 2.0;

                let minx = stransform.translation.x - hw;
                let maxx = stransform.translation.x + hw;
                let miny = stransform.translation.y - hh;
                let maxy = stransform.translation.y + hh;

                let (minx, maxx, miny, maxy) =
                    collider.adjust_container_bounds(minx, maxx, miny, maxy, BOUNDS_OFFSET);
                rigidbody.contain(&mut transform, minx, maxx, miny, maxy);
                rigidbody.repel(&mut transform, minx, maxx, miny, maxy);
            }
        }
    }
}
