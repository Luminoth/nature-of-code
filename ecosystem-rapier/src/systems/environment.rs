//! Environment systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
    simulation: Res<SimulationParams>,
    mut query: Query<
        (
            &ColliderPositionComponent,
            &ColliderShapeComponent,
            &mut WaterCurrent,
        ),
        Without<Creature>,
    >,
    mut creatures: Query<
        (
            &mut RigidBodyForcesComponent,
            &ColliderPositionComponent,
            &ColliderShapeComponent,
        ),
        With<Creature>,
    >,
) {
    if !simulation.enable_current {
        return;
    }

    for (cposition, cshape, mut current) in query.iter_mut() {
        let force = current.force(&noise);

        for (mut rbforces, ccposition, ccshape) in creatures.iter_mut() {
            let cbounds = ccshape.compute_aabb(ccposition);
            if cshape.compute_aabb(cposition).intersects(&cbounds) {
                rbforces.force += Vector::<Real>::from(force);
            }
        }

        current.update();
    }
}

/// Wind
pub fn wind(
    noise: Res<PerlinNoise>,
    simulation: Res<SimulationParams>,
    mut query: Query<
        (
            &ColliderPositionComponent,
            &ColliderShapeComponent,
            &mut Wind,
        ),
        Without<Creature>,
    >,
    mut creatures: Query<
        (
            &mut RigidBodyForcesComponent,
            &ColliderPositionComponent,
            &ColliderShapeComponent,
        ),
        With<Creature>,
    >,
) {
    if !simulation.enable_wind {
        return;
    }

    for (cposition, cshape, mut wind) in query.iter_mut() {
        let force = wind.force(&noise);

        for (mut rbforces, ccposition, ccshape) in creatures.iter_mut() {
            let cbounds = ccshape.compute_aabb(ccposition);
            if cshape.compute_aabb(cposition).intersects(&cbounds) {
                rbforces.force += Vector::<Real>::from(force);
            }
        }

        wind.update();
    }
}
