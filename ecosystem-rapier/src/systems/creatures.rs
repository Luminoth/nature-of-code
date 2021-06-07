//! Creature systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::components::physics::*;
use crate::components::*;
use crate::resources::*;

/// Creature systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CreaturesSystem {
    Think,
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
    mut query: Query<(&mut Transform, &RigidBodyVelocity, &Creature)>,
) {
    let dt = time.delta_seconds();

    for (mut transform, velocity, creature) in query.iter_mut() {
        if velocity.linvel.magnitude_squared() != 0.0 {
            let angle = -creature.acceleration_direction.angle_between(Vec2::Y);
            transform.rotation = transform.rotation.slerp(Quat::from_rotation_z(angle), dt);
        }
    }
}

/// Fly behavior
pub fn fly_update(mut query: Query<&mut Fly>) {
    for mut _fly in query.iter_mut() {}
}

/// Fly behavior
pub fn fly_think(mut random: ResMut<Random>, mut query: Query<&mut Creature, With<Fly>>) {
    for mut creature in query.iter_mut() {
        creature.acceleration_direction = random.direction();
    }
}

/// Fly behavior
pub fn fly_physics(
    mut random: ResMut<Random>,
    mut query: Query<(&mut RigidBodyForces, &RigidBodyMassProps, &Fly, &Creature)>,
) {
    for (mut forces, mass, fly, creature) in query.iter_mut() {
        let _modifier = random.random() as f32;
        let magnitude = fly.acceleration * mass.mass(); // * _modifier;

        forces.force = (creature.acceleration_direction * magnitude).into();
    }
}

/// Keep flies inside the window
pub fn fly_bounds(
    world_bounds: Res<WorldBounds>,
    mut query: Query<(&mut Transform, &ColliderPosition, &ColliderShape, &Physical), With<Fly>>,
) {
    let hw = world_bounds.width / 2.0;
    let hh = world_bounds.height / 2.0;

    for (mut transform, collider_position, shape, physical) in query.iter_mut() {
        let bounds = shape.compute_aabb(collider_position);

        let (min, max) = adjust_container_bounds(
            bounds.extents().into(),
            Vec2::new(-hw, -hh),
            Vec2::new(hw, hh),
            BOUNDS_OFFSET,
        );

        let rigidbody = rigidbodies.get_mut(collider.parent()).unwrap();
        contain(rigidbody, &mut transform, physical, min, max, FLY_SIZE);
        bounds_repel(
            rigidbody,
            &transform,
            min,
            max,
            BOUNDS_REPEL_ACCEL,
            FLY_SIZE,
        );
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

            let frigidbody = rigidbodies.get_mut(frbhandle.handle()).unwrap();
            repel(
                frigidbody,
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
pub fn fish_think(
    mut random: ResMut<Random>,
    rigidbodies: Res<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &mut Creature), With<Fish>>,
) {
    for (rbhandle, mut creature) in query.iter_mut() {
        let rigidbody = rigidbodies.get(rbhandle.handle()).unwrap();
        creature.acceleration_direction = if rigidbody.linvel().magnitude_squared() == 0.0 {
            random.direction()
        } else {
            let direction = to_vector(random.direction());
            from_vector(rigidbody.linvel().lerp(&direction, THINK_STEP).normalize())
        };
    }
}

/// Fish behavior
pub fn fish_physics(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Fish, &Creature)>,
) {
    for (rbhandle, fish, creature) in query.iter_mut() {
        let rigidbody = rigidbodies.get_mut(rbhandle.handle()).unwrap();

        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
        let _modifier = noise.get(t, random.random_range(0.5..0.75)) as f32;
        let magnitude = fish.acceleration * rigidbody.mass(); // * _modifier;

        rigidbody.apply_force(to_vector(creature.acceleration_direction * magnitude), true);
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

            let frigidbody = rigidbodies.get_mut(frbhandle.handle()).unwrap();
            repel(
                frigidbody,
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
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&mut Transform, &ColliderHandleComponent, &Physical), With<Fish>>,
    waters: Query<(&Transform, &ColliderHandleComponent), (With<Water>, Without<Fish>)>,
) {
    for (mut transform, chandle, physical) in query.iter_mut() {
        for (wtransform, wchandle) in waters.iter() {
            let wcollider = colliders.get(wchandle.handle()).unwrap();
            let wbounds = wcollider.compute_aabb();

            let collider = colliders.get(chandle.handle()).unwrap();
            if collider.compute_aabb().intersects(&wbounds) {
                let position = wtransform.translation.truncate();
                let half_size = from_vector(wbounds.half_extents());

                let min = position - half_size;
                let max = position + half_size;

                let (min, max) = adjust_container_bounds(
                    from_vector(wbounds.extents()),
                    min,
                    max,
                    BOUNDS_OFFSET,
                );

                let rigidbody = rigidbodies.get_mut(collider.parent()).unwrap();
                contain(rigidbody, &mut transform, physical, min, max, FISH_WIDTH);
                bounds_repel(
                    rigidbody,
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

/// Snake behavior
pub fn snake_update(mut query: Query<&mut Snake>) {
    for mut _snake in query.iter_mut() {}
}

/// Snake behavior
pub fn snake_think(
    mut random: ResMut<Random>,
    rigidbodies: Res<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &mut Creature), With<Snake>>,
) {
    for (rbhandle, mut creature) in query.iter_mut() {
        let rigidbody = rigidbodies.get(rbhandle.handle()).unwrap();
        creature.acceleration_direction = if rigidbody.linvel().magnitude_squared() == 0.0 {
            random.direction()
        } else {
            let direction = to_vector(random.direction());
            from_vector(rigidbody.linvel().lerp(&direction, THINK_STEP).normalize())
        };
    }
}

/// Snake behavior
pub fn snake_physics(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Snake, &Creature)>,
) {
    for (rbhandle, snake, creature) in query.iter_mut() {
        let rigidbody = rigidbodies.get_mut(rbhandle.handle()).unwrap();

        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
        let _modifier = noise.get(t, random.random_range(0.25..0.5)) as f32;
        let magnitude = snake.ground_acceleration * rigidbody.mass(); // * _modifier;

        rigidbody.apply_force(to_vector(creature.acceleration_direction * magnitude), true);
    }
}

/// Keep snakes on the ground (for now)
pub fn snake_bounds(
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&mut Transform, &ColliderHandleComponent, &Physical), With<Snake>>,
    grounds: Query<(&Transform, &ColliderHandleComponent), (With<Ground>, Without<Snake>)>,
) {
    for (mut transform, chandle, physical) in query.iter_mut() {
        for (gtransform, gchandle) in grounds.iter() {
            let gcollider = colliders.get(gchandle.handle()).unwrap();
            let gbounds = gcollider.compute_aabb();

            let collider = colliders.get(chandle.handle()).unwrap();
            if collider.compute_aabb().intersects(&gbounds) {
                let position = gtransform.translation.truncate();
                let half_size = from_vector(gbounds.half_extents());

                let min = position - half_size;
                let max = position + half_size;

                let (min, max) = adjust_container_bounds(
                    from_vector(gbounds.extents()),
                    min,
                    max,
                    BOUNDS_OFFSET,
                );

                let rigidbody = rigidbodies.get_mut(collider.parent()).unwrap();
                contain(rigidbody, &mut transform, physical, min, max, SNAKE_WIDTH);
                bounds_repel(
                    rigidbody,
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

/// Snakes repel each other
pub fn snake_repel(
    mut rigidbodies: ResMut<RigidBodySet>,
    query: Query<(Entity, &Transform, &Snake)>,
    mut squery: Query<(Entity, &Transform, &RigidBodyHandleComponent), With<Snake>>,
) {
    for (entity, transform, snake) in query.iter() {
        for (sentity, stransform, srbhandle) in squery.iter_mut() {
            if entity == sentity {
                continue;
            }

            let srigidbody = rigidbodies.get_mut(srbhandle.handle()).unwrap();
            repel(
                srigidbody,
                transform,
                stransform.translation.truncate(),
                snake.repel_acceleration,
                SNAKE_WIDTH,
            );
        }
    }
}
