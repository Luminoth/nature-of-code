//! Environment systems

use bevy::prelude::*;
use bevy_rapier2d::physics::ColliderHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::ColliderSet;
use bevy_rapier2d::rapier::parry::bounding_volume::BoundingVolume;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::resources::*;
use crate::util::to_vector;

/// Environment systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum EnvironmentsSystem {
    Physics,
}

/// Water current
pub fn water_current(
    noise: Res<PerlinNoise>,
    simulation: Res<SimulationParams>,
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&ColliderHandleComponent, &mut WaterCurrent), Without<Creature>>,
    creatures: Query<&ColliderHandleComponent, With<Creature>>,
) {
    if !simulation.enable_current {
        return;
    }

    for (chandle, mut current) in query.iter_mut() {
        let collider = colliders.get(chandle.handle()).unwrap();
        let force = current.force(&noise);

        for cchandle in creatures.iter() {
            let ccollider = colliders.get(cchandle.handle()).unwrap();
            let rigidbody = rigidbodies.get_mut(ccollider.parent()).unwrap();

            let cbounds = ccollider.compute_aabb();
            if collider.compute_aabb().intersects(&cbounds) {
                rigidbody.apply_force(to_vector(force), true);
            }
        }

        current.update();
    }
}

/// Wind
pub fn wind(
    noise: Res<PerlinNoise>,
    simulation: Res<SimulationParams>,
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&ColliderHandleComponent, &mut Wind), Without<Creature>>,
    creatures: Query<&ColliderHandleComponent, With<Creature>>,
) {
    if !simulation.enable_wind {
        return;
    }

    for (chandle, mut wind) in query.iter_mut() {
        let collider = colliders.get(chandle.handle()).unwrap();
        let force = wind.force(&noise);

        for cchandle in creatures.iter() {
            let ccollider = colliders.get(cchandle.handle()).unwrap();
            let rigidbody = rigidbodies.get_mut(ccollider.parent()).unwrap();

            let cbounds = ccollider.compute_aabb();
            if collider.compute_aabb().intersects(&cbounds) {
                rigidbody.apply_force(to_vector(force), true);
            }
        }

        wind.update();
    }
}
