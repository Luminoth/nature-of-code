//! Environment systems

use bevy::prelude::*;
use bevy_rapier2d::physics::ColliderHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::ColliderSet;
use bevy_rapier2d::rapier::parry::bounding_volume::BoundingVolume;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::resources::*;

/// Environment systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum EnvironmentsSystem {
    Physics,
}

/// Water current
pub fn water_current(
    noise: Res<PerlinNoise>,
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&ColliderHandleComponent, &mut WaterCurrent), Without<Creature>>,
    creatures: Query<&ColliderHandleComponent, With<Creature>>,
) {
    for (chandle, mut current) in query.iter_mut() {
        if let Some(collider) = colliders.get(chandle.handle()) {
            let force = current.force(&noise);

            for cchandle in creatures.iter() {
                if let Some(ccollider) = colliders.get(cchandle.handle()) {
                    if let Some(rigidbody) = rigidbodies.get_mut(ccollider.parent()) {
                        let cbounds = ccollider.compute_aabb();
                        if collider.compute_aabb().intersects(&cbounds) {
                            rigidbody.apply_force(force.extend(0.0), true);
                        }
                    }
                }
            }

            current.update();
        }
    }
}

/// Wind
pub fn wind(
    noise: Res<PerlinNoise>,
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&ColliderHandleComponent, &mut Wind), Without<Creature>>,
    creatures: Query<&ColliderHandleComponent, With<Creature>>,
) {
    for (chandle, mut wind) in query.iter_mut() {
        if let Some(collider) = colliders.get(chandle.handle()) {
            let force = wind.force(&noise);

            for cchandle in creatures.iter() {
                if let Some(ccollider) = colliders.get(cchandle.handle()) {
                    if let Some(rigidbody) = rigidbodies.get_mut(ccollider.parent()) {
                        let cbounds = ccollider.compute_aabb();
                        if collider.compute_aabb().intersects(&cbounds) {
                            rigidbody.apply_force(force, true);
                        }
                    }
                }
            }

            wind.update();
        }
    }
}
