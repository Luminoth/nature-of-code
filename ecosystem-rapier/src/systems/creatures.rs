//! Creature systems

use bevy::prelude::*;

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
pub fn creature_facing(time: Res<Time>, mut query: Query<(&mut Transform, &Rigidbody, &Creature)>) {
    for (mut transform, rigidbody, creature) in query.iter_mut() {
        let angle = -creature.acceleration_direction.angle_between(Vec2::Y);
        if rigidbody.velocity.length_squared() != 0.0 {
            transform.rotation = transform
                .rotation
                .slerp(Quat::from_rotation_z(angle), time.delta_seconds());
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
    mut query: Query<(&mut Rigidbody, &Fly, &mut Creature)>,
) {
    for (mut rigidbody, fly, mut creature) in query.iter_mut() {
        let _modifier = random.random() as f32;

        creature.acceleration_direction = random.direction();
        let magnitude = fly.acceleration * rigidbody.mass; // * _modifier;
        rigidbody.apply_force(creature.acceleration_direction * magnitude);
    }
}

/// Keep flies inside the window
pub fn fly_bounds(
    world_bounds: Res<WorldBounds>,
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Fly>>,
) {
    let hw = world_bounds.width / 2.0;
    let hh = world_bounds.height / 2.0;

    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        let (min, max) =
            collider.adjust_container_bounds(Vec2::new(-hw, -hh), Vec2::new(hw, hh), BOUNDS_OFFSET);
        rigidbody.contain(&mut transform, min, max, FLY_SIZE);
        rigidbody.bounds_repel(&transform, min, max, BOUNDS_REPEL_ACCEL, FLY_SIZE);
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
                FLY_SIZE,
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
    mut query: Query<(&mut Rigidbody, &Fish, &mut Creature)>,
) {
    for (mut rigidbody, fish, mut creature) in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
        let _modifier = noise.get(t, random.random_range(0.5..0.75)) as f32;

        creature.acceleration_direction = if rigidbody.velocity.length_squared() == 0.0 {
            random.direction()
        } else {
            rigidbody
                .velocity
                .truncate()
                .lerp(random.direction(), PHYSICS_STEP)
                .normalize()
        };
        let magnitude = fish.acceleration * rigidbody.mass; // * _modifier;
        rigidbody.apply_force(creature.acceleration_direction * magnitude);
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
                FISH_WIDTH,
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
            if collider.collides(&transform, (ftransform, fcollider)) {
                let position = ftransform.translation.truncate();
                let half_size = fcollider.size() / 2.0;

                let min = position - half_size;
                let max = position + half_size;

                let (min, max) = collider.adjust_container_bounds(min, max, BOUNDS_OFFSET);
                rigidbody.contain(&mut transform, min, max, FISH_WIDTH);
                rigidbody.bounds_repel(&transform, min, max, BOUNDS_REPEL_ACCEL, FISH_WIDTH);
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
    mut query: Query<(&mut Rigidbody, &Snake, &mut Creature)>,
) {
    for (mut rigidbody, snake, mut creature) in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
        let _modifier = noise.get(t, random.random_range(0.25..0.5)) as f32;

        creature.acceleration_direction = if rigidbody.velocity.length_squared() == 0.0 {
            random.direction()
        } else {
            rigidbody
                .velocity
                .truncate()
                .lerp(random.direction(), PHYSICS_STEP)
                .normalize()
        };
        let magnitude = snake.ground_acceleration * rigidbody.mass; // * _modifier;
        rigidbody.apply_force(creature.acceleration_direction * magnitude);
    }
}

/// Keep snakes on the ground (for now)
pub fn snake_bounds(
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Snake>>,
    surfaces: Query<(&Transform, &Collider), (With<Surface>, Without<Snake>)>,
) {
    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        for (stransform, scollider) in surfaces.iter() {
            if collider.collides(&transform, (stransform, scollider)) {
                let position = stransform.translation.truncate();
                let half_size = scollider.size() / 2.0;

                let min = position - half_size;
                let max = position + half_size;

                let (min, max) = collider.adjust_container_bounds(min, max, BOUNDS_OFFSET);
                rigidbody.contain(&mut transform, min, max, SNAKE_WIDTH);
                rigidbody.bounds_repel(&transform, min, max, BOUNDS_REPEL_ACCEL, SNAKE_WIDTH);
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
                SNAKE_WIDTH,
            );
        }
    }
}
