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

// TODO: move these to SimulationParams
const BOUNDS_OFFSET: f32 = 5.0;
const BOUNDS_REPEL_ACCEL: f32 = 10.0;

/// Creature facing
pub fn creature_facing(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Rigidbody), With<Creature>>,
) {
    for (mut transform, rigidbody) in query.iter_mut() {
        if rigidbody.velocity.length_squared() != 0.0 {
            transform.rotation = transform.rotation.slerp(
                Quat::from_rotation_z(rigidbody.velocity.angle_between(Vec3::Y)),
                time.delta_seconds(),
            );
        }
    }
}

/// Fly behavior
pub fn fly_update(mut query: Query<&mut Fly>) {
    for mut _fly in query.iter_mut() {}
}

/// Fly behavior
pub fn fly_physics(mut random: ResMut<Random>, mut query: Query<(&mut Rigidbody, &Fly)>) {
    for (mut rigidbody, fly) in query.iter_mut() {
        let direction = random.direction();
        let modifier = random.random() as f32;
        let magnitude = fly.acceleration * rigidbody.mass * modifier;
        rigidbody.apply_force(direction * magnitude);
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
        rigidbody.bounds_repel(&transform, minx, maxx, miny, maxy, BOUNDS_REPEL_ACCEL);
    }
}

/// Flies repel each other
pub fn fly_repel(
    query: Query<(Entity, &Transform, &Fly)>,
    mut fquery: Query<(Entity, &Transform, &mut Rigidbody), With<Fly>>,
) {
    for (entity, transform, fish) in query.iter() {
        for (fentity, ftransform, mut frigidbody) in fquery.iter_mut() {
            if entity == fentity {
                continue;
            }

            frigidbody.repel(
                transform,
                ftransform.translation.truncate(),
                fish.repel_acceleration,
            );
        }
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
    mut query: Query<(&mut Rigidbody, &Fish)>,
) {
    for (mut rigidbody, fish) in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);

        let direction = random.direction();
        let modifier = noise.get(t, 0.5) as f32;
        let magnitude = fish.acceleration * rigidbody.mass * modifier;
        rigidbody.apply_force(direction * magnitude);
    }
}

/// Fish repel each other
pub fn fish_repel(
    query: Query<(Entity, &Transform, &Fish)>,
    mut fquery: Query<(Entity, &Transform, &mut Rigidbody), With<Fish>>,
) {
    for (entity, transform, fish) in query.iter() {
        for (fentity, ftransform, mut frigidbody) in fquery.iter_mut() {
            if entity == fentity {
                continue;
            }

            frigidbody.repel(
                transform,
                ftransform.translation.truncate(),
                fish.repel_acceleration,
            );
        }
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
                rigidbody.bounds_repel(&transform, minx, maxx, miny, maxy, BOUNDS_REPEL_ACCEL);
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
    mut query: Query<(&mut Rigidbody, &Snake)>,
) {
    for (mut rigidbody, snake) in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);

        let direction = random.direction();
        let modifier = noise.get(t, 0.5) as f32;
        let magnitude = snake.ground_acceleration * rigidbody.mass * modifier;
        rigidbody.apply_force(direction * magnitude);
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
                rigidbody.bounds_repel(&transform, minx, maxx, miny, maxy, BOUNDS_REPEL_ACCEL);
            }
        }
    }
}

/// Snakes repel each other
pub fn snake_repel(
    query: Query<(Entity, &Transform, &Snake)>,
    mut squery: Query<(Entity, &Transform, &mut Rigidbody), With<Snake>>,
) {
    for (entity, transform, fish) in query.iter() {
        for (sentity, stransform, mut srigidbody) in squery.iter_mut() {
            if entity == sentity {
                continue;
            }

            srigidbody.repel(
                transform,
                stransform.translation.truncate(),
                fish.repel_acceleration,
            );
        }
    }
}
