//! Physics systems

use bevy::prelude::*;

use crate::components::physics::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct Physics;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    Collisions,
    Update,
}

/// Handles physics collisions
// TODO: this name sucks
pub fn physics_collisions(
    mut query: Query<(&Transform, &mut Rigidbody, &Collider)>,
    surfaces: Query<(&Surface, &Transform, &Collider)>,
    fluids: Query<(&Fluid, &Transform, &Collider)>,
) {
    for (transform, mut rigidbody, collider) in query.iter_mut() {
        for (surface, stransform, scollider) in surfaces.iter() {
            if collider.collides(transform, (stransform, scollider)) {
                surface.update(&mut rigidbody);
            }
        }

        for (fluid, ftransform, fcollider) in fluids.iter() {
            if collider.collides(transform, (ftransform, fcollider)) {
                fluid.update(&mut rigidbody);
            }
        }
    }
}

/// Updates rigidbodies and applies transform changes
pub fn physics_update(mut query: Query<(&mut Transform, &mut Rigidbody)>) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        rigidbody.update(&mut transform);
    }
}

/// Updates an oscillator
pub fn oscillator_update(mut query: Query<(&mut Transform, &mut Oscillator)>) {
    for (mut transform, mut oscillator) in query.iter_mut() {
        oscillator.update(&mut transform);
    }
}
