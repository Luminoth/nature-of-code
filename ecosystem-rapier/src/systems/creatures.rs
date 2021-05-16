//! Creature systems

use bevy::prelude::*;
use bevy_rapier2d::physics::{ColliderHandleComponent, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::ColliderSet;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::resources::*;

/// Creature systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CreaturesSystem {
    Update,
    UpdateAfter,
    Physics,
    Bounds,
}

// TODO: move these to SimulationParams
const BOUNDS_OFFSET: f32 = 0.1;
const BOUNDS_REPEL_ACCEL: f32 = 0.01;

/// Creature facing
pub fn creature_facing(
    time: Res<Time>,
    rigidbodies: Res<RigidBodySet>,
    mut query: Query<(&mut Transform, &RigidBodyHandleComponent, &Creature)>,
) {
    for (mut transform, rbhandle, creature) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get(rbhandle.handle()) {
            let angle = -creature.acceleration_direction.angle_between(Vec2::Y);
            if rigidbody.linvel().magnitude_squared() != 0.0 {
                transform.rotation = transform
                    .rotation
                    .slerp(Quat::from_rotation_z(angle), time.delta_seconds());
            }
        }
    }
}

/// Fly behavior
pub fn fly_update(mut query: Query<&mut Fly>) {
    for mut _fly in query.iter_mut() {}
}

/// Fly behavior
pub fn fly_physics(
    mut random: ResMut<Random>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Fly, &mut Creature)>,
) {
    for (mut rbhandle, fly, mut creature) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            let _modifier = random.random() as f32;

            creature.acceleration_direction = random.direction();
            let magnitude = fly.acceleration * rigidbody.mass(); // * _modifier;
            rigidbody.apply_force(creature.acceleration_direction * magnitude, true);
        }
    }
}

/// Keep flies inside the window
pub fn fly_bounds(
    world_bounds: Res<WorldBounds>,
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<
        (
            &mut Transform,
            &RigidBodyHandleComponent,
            &ColliderHandleComponent,
        ),
        With<Fly>,
    >,
) {
    let hw = world_bounds.width / 2.0;
    let hh = world_bounds.height / 2.0;

    for (mut transform, rbhandle, chandle) in query.iter_mut() {
        if let Some(collider) = colliders.get(chandle.handle()) {
            if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
                let (min, max) = collider.adjust_container_bounds(
                    Vec2::new(-hw, -hh),
                    Vec2::new(hw, hh),
                    BOUNDS_OFFSET,
                );

                rigidbody.contain(&mut transform, min, max, FLY_SIZE);
                rigidbody.bounds_repel(&transform, min, max, BOUNDS_REPEL_ACCEL, FLY_SIZE);
            }
        }
    }
}

/// Flies repel each other
pub fn fly_repel(
    mut rigidbodies: ResMut<RigidBodySet>,
    query: Query<(Entity, &Transform, &Fly)>,
    mut fquery: Query<(Entity, &Transform, &RigidBodyHandleComponent), With<Fly>>,
) {
    for (entity, transform, fish) in query.iter() {
        for (fentity, ftransform, frbhandle) in fquery.iter_mut() {
            if entity == fentity {
                continue;
            }

            if let Some(frigidbody) = rigidbodies.get_mut(frbhandle.handle()) {
                frigidbody.repel(
                    transform,
                    ftransform.translation.truncate(),
                    fish.repel_acceleration,
                    FLY_SIZE,
                );
            }
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
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Fish, &mut Creature)>,
) {
    for (rbhandle, fish, mut creature) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
            let _modifier = noise.get(t, random.random_range(0.5..0.75)) as f32;

            creature.acceleration_direction = if rigidbody.linvel().magnitude_squared() == 0.0 {
                random.direction()
            } else {
                rigidbody
                    .linvel()
                    .truncate()
                    .lerp(random.direction(), PHYSICS_STEP)
                    .normalize()
            };

            let magnitude = fish.acceleration * rigidbody.mass(); // * _modifier;

            rigidbody.apply_force(creature.acceleration_direction * magnitude, true);
        }
    }
}

/// Fish repel each other
pub fn fish_repel(
    mut rigidbodies: ResMut<RigidBodySet>,
    query: Query<(Entity, &Transform, &Fish)>,
    mut fquery: Query<(Entity, &Transform, &RigidBodyHandleComponent), With<Fish>>,
) {
    for (entity, transform, fish) in query.iter() {
        for (fentity, ftransform, frbhandle) in fquery.iter_mut() {
            if entity == fentity {
                continue;
            }

            if let Some(frigidbody) = rigidbodies.get_mut(frbhandle.handle()) {
                frigidbody.repel(
                    transform,
                    ftransform.translation.truncate(),
                    fish.repel_acceleration,
                    FISH_WIDTH,
                );
            }
        }
    }
}

/// Keep fish inside the water
pub fn fish_bounds(
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<
        (
            &mut Transform,
            &RigidBodyHandleComponent,
            &ColliderHandleComponent,
        ),
        With<Fish>,
    >,
    fluids: Query<(&Transform, &ColliderHandleComponent), (With<Fluid>, Without<Fish>)>,
) {
    for (mut transform, rbhandle, chandle) in query.iter_mut() {
        for (ftransform, fchandle) in fluids.iter() {
            if let Some(collider) = colliders.get(chandle.handle()) {
                if let Some(fcollider) = colliders.get(fchandle.handle()) {
                    if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
                        if collider.collides(&transform, (ftransform, fcollider)) {
                            let position = ftransform.translation.truncate();
                            let half_size = fcollider.size() / 2.0;

                            let min = position - half_size;
                            let max = position + half_size;

                            let (min, max) =
                                collider.adjust_container_bounds(min, max, BOUNDS_OFFSET);
                            rigidbody.contain(&mut transform, min, max, FISH_WIDTH);
                            rigidbody.bounds_repel(
                                &transform,
                                min,
                                max,
                                BOUNDS_REPEL_ACCEL,
                                FISH_WIDTH,
                            );
                        }
                    }
                }
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
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Snake, &mut Creature)>,
) {
    for (rbhandle, snake, mut creature) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
            let _modifier = noise.get(t, random.random_range(0.25..0.5)) as f32;

            creature.acceleration_direction = if rigidbody.linvel().magnitude_squared() == 0.0 {
                random.direction()
            } else {
                rigidbody
                    .linvel()
                    .truncate()
                    .lerp(random.direction(), PHYSICS_STEP)
                    .normalize()
            };

            let magnitude = snake.ground_acceleration * rigidbody.mass(); // * _modifier;

            rigidbody.apply_force(creature.acceleration_direction * magnitude, true);
        }
    }
}

/// Keep snakes on the ground (for now)
pub fn snake_bounds(
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<
        (
            &mut Transform,
            &RigidBodyHandleComponent,
            &ColliderHandleComponent,
        ),
        With<Snake>,
    >,
    surfaces: Query<(&Transform, &ColliderHandleComponent), (With<Surface>, Without<Snake>)>,
) {
    for (mut transform, rbhandle, chandle) in query.iter_mut() {
        for (stransform, schandle) in surfaces.iter() {
            if let Some(collider) = colliders.get(chandle.handle()) {
                if let Some(scollider) = colliders.get(schandle.handle()) {
                    if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
                        if collider.collides(&transform, (stransform, scollider)) {
                            let position = stransform.translation.truncate();
                            let half_size = scollider.size() / 2.0;

                            let min = position - half_size;
                            let max = position + half_size;

                            let (min, max) =
                                collider.adjust_container_bounds(min, max, BOUNDS_OFFSET);
                            rigidbody.contain(&mut transform, min, max, SNAKE_WIDTH);
                            rigidbody.bounds_repel(
                                &transform,
                                min,
                                max,
                                BOUNDS_REPEL_ACCEL,
                                SNAKE_WIDTH,
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Snakes repel each other
pub fn snake_repel(
    mut rigidbodies: ResMut<RigidBodySet>,
    query: Query<(Entity, &Transform, &Snake)>,
    mut squery: Query<(Entity, &Transform, &RigidBodyHandleComponent), With<Snake>>,
) {
    for (entity, transform, fish) in query.iter() {
        for (sentity, stransform, srbhandle) in squery.iter_mut() {
            if entity == sentity {
                continue;
            }

            if let Some(srigidbody) = rigidbodies.get_mut(srbhandle.handle()) {
                srigidbody.repel(
                    transform,
                    stransform.translation.truncate(),
                    fish.repel_acceleration,
                    SNAKE_WIDTH,
                );
            }
        }
    }
}
