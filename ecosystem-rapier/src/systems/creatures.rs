//! Creature systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::parry::bounding_volume::BoundingVolume;

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

    for (mut transform, rbvelocity, creature) in query.iter_mut() {
        if rbvelocity.linvel.magnitude_squared() != 0.0 {
            let angle = -creature.acceleration_direction.angle_between(Vec2::Y);
            // TODO: this is wrong, we have to modify something else
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
    mut query: Query<(&RigidBodyMassProps, &mut RigidBodyForces, &Fly, &Creature)>,
) {
    for (rbmass, mut forces, fly, creature) in query.iter_mut() {
        let _modifier = random.random() as f32;
        let magnitude = fly.acceleration * rbmass.mass(); // * _modifier;

        forces.force += Vector::<Real>::from(creature.acceleration_direction * magnitude);
    }
}

/// Keep flies inside the window
pub fn fly_bounds(
    world_bounds: Res<WorldBounds>,
    mut query: Query<
        (
            &mut Transform,
            &RigidBodyMassProps,
            &mut RigidBodyVelocity,
            &mut RigidBodyForces,
            &ColliderPosition,
            &ColliderShape,
            &Physical,
        ),
        With<Fly>,
    >,
) {
    let hw = world_bounds.width / 2.0;
    let hh = world_bounds.height / 2.0;

    for (mut transform, rbmass, mut rbvelocity, mut rbforces, cposition, cshape, physical) in
        query.iter_mut()
    {
        let bounds = cshape.compute_aabb(cposition);

        let (min, max) = adjust_container_bounds(
            bounds.extents().into(),
            Vec2::new(-hw, -hh),
            Vec2::new(hw, hh),
            BOUNDS_OFFSET,
        );

        contain(
            &mut rbvelocity,
            &mut transform,
            physical,
            min,
            max,
            FLY_SIZE,
        );

        bounds_repel(
            rbmass,
            &mut rbforces,
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
    query: Query<(Entity, &Transform, &Fly)>,
    mut fquery: Query<
        (
            Entity,
            &Transform,
            &RigidBodyMassProps,
            &mut RigidBodyForces,
        ),
        With<Fly>,
    >,
) {
    for (entity, transform, fish) in query.iter() {
        for (fentity, ftransform, rbmass, mut rbforces) in fquery.iter_mut() {
            if entity == fentity {
                continue;
            }

            repel(
                rbmass,
                &mut rbforces,
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
    mut query: Query<(&RigidBodyVelocity, &mut Creature), With<Fish>>,
) {
    for (rbvelocity, mut creature) in query.iter_mut() {
        creature.acceleration_direction = if rbvelocity.linvel.magnitude_squared() == 0.0 {
            random.direction()
        } else {
            let direction = random.direction().into();
            rbvelocity
                .linvel
                .lerp(&direction, THINK_STEP)
                .normalize()
                .into()
        };
    }
}

/// Fish behavior
pub fn fish_physics(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&RigidBodyMassProps, &mut RigidBodyForces, &Fish, &Creature)>,
) {
    for (rbmass, mut rbforces, fish, creature) in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
        let _modifier = noise.get(t, random.random_range(0.5..0.75)) as f32;
        let magnitude = fish.acceleration * rbmass.mass(); // * _modifier;

        rbforces.force += Vector::<Real>::from(creature.acceleration_direction * magnitude);
    }
}

/// Keep fish inside the water
pub fn fish_bounds(
    mut query: Query<
        (
            &mut Transform,
            &RigidBodyMassProps,
            &mut RigidBodyVelocity,
            &mut RigidBodyForces,
            &ColliderPosition,
            &ColliderShape,
            &Physical,
        ),
        With<Fish>,
    >,
    waters: Query<(&Transform, &ColliderPosition, &ColliderShape), (With<Water>, Without<Fish>)>,
) {
    for (mut transform, rbmass, mut rbvelocity, mut rbforces, cposition, cshape, physical) in
        query.iter_mut()
    {
        for (wtransform, wcposition, wcshape) in waters.iter() {
            let wbounds = wcshape.compute_aabb(wcposition);

            if cshape.compute_aabb(cposition).intersects(&wbounds) {
                let position = wtransform.translation.truncate();
                let half_size = wbounds.half_extents().into();

                let min = position - half_size;
                let max = position + half_size;

                let (min, max) =
                    adjust_container_bounds(wbounds.extents().into(), min, max, BOUNDS_OFFSET);

                contain(
                    &mut rbvelocity,
                    &mut transform,
                    physical,
                    min,
                    max,
                    FISH_WIDTH,
                );

                bounds_repel(
                    rbmass,
                    &mut rbforces,
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

/// Fish repel each other
pub fn fish_repel(
    query: Query<(Entity, &Transform, &Fish)>,
    mut fquery: Query<
        (
            Entity,
            &Transform,
            &RigidBodyMassProps,
            &mut RigidBodyForces,
        ),
        With<Fish>,
    >,
) {
    for (entity, transform, fish) in query.iter() {
        for (fentity, ftransform, frbmass, mut frbforces) in fquery.iter_mut() {
            if entity == fentity {
                continue;
            }

            repel(
                frbmass,
                &mut frbforces,
                transform,
                ftransform.translation.truncate(),
                fish.repel_acceleration,
                FISH_WIDTH,
            );
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
    mut query: Query<(&RigidBodyVelocity, &mut Creature), With<Snake>>,
) {
    for (rbvelocity, mut creature) in query.iter_mut() {
        creature.acceleration_direction = if rbvelocity.linvel.magnitude_squared() == 0.0 {
            random.direction()
        } else {
            let direction = random.direction().into();
            rbvelocity
                .linvel
                .lerp(&direction, THINK_STEP)
                .normalize()
                .into()
        };
    }
}

/// Snake behavior
pub fn snake_physics(
    time: Res<Time>,
    mut random: ResMut<Random>,
    noise: Res<PerlinNoise>,
    mut query: Query<(&RigidBodyMassProps, &mut RigidBodyForces, &Snake, &Creature)>,
) {
    for (rbmass, mut rbforces, snake, creature) in query.iter_mut() {
        let t = time.seconds_since_startup() + random.random_range(0.0..0.5);
        let _modifier = noise.get(t, random.random_range(0.25..0.5)) as f32;
        let magnitude = snake.ground_acceleration * rbmass.mass(); // * _modifier;

        rbforces.force += Vector::<Real>::from(creature.acceleration_direction * magnitude);
    }
}

/// Keep snakes on the ground (for now)
pub fn snake_bounds(
    mut query: Query<
        (
            &mut Transform,
            &RigidBodyMassProps,
            &mut RigidBodyVelocity,
            &mut RigidBodyForces,
            &ColliderPosition,
            &ColliderShape,
            &Physical,
        ),
        With<Snake>,
    >,
    grounds: Query<(&Transform, &ColliderPosition, &ColliderShape), (With<Ground>, Without<Snake>)>,
) {
    for (mut transform, rbmass, mut rbvelocity, mut rbforces, cposition, cshape, physical) in
        query.iter_mut()
    {
        for (gtransform, gcposition, gcshape) in grounds.iter() {
            let gbounds = gcshape.compute_aabb(gcposition);

            if cshape.compute_aabb(cposition).intersects(&gbounds) {
                let position = gtransform.translation.truncate();
                let half_size = gbounds.half_extents().into();

                let min = position - half_size;
                let max = position + half_size;

                let (min, max) =
                    adjust_container_bounds(gbounds.extents().into(), min, max, BOUNDS_OFFSET);

                contain(
                    &mut rbvelocity,
                    &mut transform,
                    physical,
                    min,
                    max,
                    SNAKE_WIDTH,
                );

                bounds_repel(
                    rbmass,
                    &mut rbforces,
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
    query: Query<(Entity, &Transform, &Snake)>,
    mut squery: Query<
        (
            Entity,
            &Transform,
            &RigidBodyMassProps,
            &mut RigidBodyForces,
        ),
        With<Snake>,
    >,
) {
    for (entity, transform, snake) in query.iter() {
        for (sentity, stransform, srbmass, mut srbforces) in squery.iter_mut() {
            if entity == sentity {
                continue;
            }

            repel(
                srbmass,
                &mut srbforces,
                transform,
                stransform.translation.truncate(),
                snake.repel_acceleration,
                SNAKE_WIDTH,
            );
        }
    }
}
