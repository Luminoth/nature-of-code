//! Environment systems

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::components::physics::*;
use crate::resources::*;

/// Environment systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum EnvironmentsSystem {
    Physics,
}

/// Water current
pub fn water_current(
    noise: Res<PerlinNoise>,
    mut query: Query<(&Transform, &Collider, &mut WaterCurrent), Without<Creature>>,
    mut creatures: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Creature>>,
) {
    for (transform, collider, mut current) in query.iter_mut() {
        let force = current.force(&noise);

        for (ctransform, mut rigidbody, ccollider) in creatures.iter_mut() {
            if collider.collides(transform, (&ctransform, ccollider)) {
                rigidbody.apply_force(force);
            }
        }

        current.update();
    }
}

/// Wind
pub fn wind(
    noise: Res<PerlinNoise>,
    mut query: Query<(&Transform, &Collider, &mut Wind), Without<Creature>>,
    mut creatures: Query<(&mut Transform, &mut Rigidbody, &Collider), With<Creature>>,
) {
    for (transform, collider, mut wind) in query.iter_mut() {
        let force = wind.force(&noise);

        for (ctransform, mut rigidbody, ccollider) in creatures.iter_mut() {
            if collider.collides(transform, (&ctransform, ccollider)) {
                rigidbody.apply_force(force);
            }
        }

        wind.update();
    }
}
