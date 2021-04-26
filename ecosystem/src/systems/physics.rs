//! Physics systems

use bevy::prelude::*;

use crate::components::physics::*;

/// Handles physics collisions
pub fn physics_collisions(
    mut query: Query<(&Transform, &mut Rigidbody, &Collider)>,
    surfaces: Query<(&Surface, &Transform, &Collider)>,
    fluids: Query<(&Fluid, &Transform, &Collider)>,
) {
    for (transform, mut rigidbody, collider) in query.iter_mut() {
        // apply friction
        for (surface, stransform, scollider) in surfaces.iter() {
            if collider.collides(transform, scollider, stransform) {
                let magnitude = surface.c;
                let friction = -rigidbody.velocity.normalize_or_zero() * magnitude;
                rigidbody.apply_force(friction.truncate());
            }
        }

        // apply drag
        for (fluid, ftransform, fcollider) in fluids.iter() {
            if collider.collides(transform, fcollider, ftransform) {
                let magnitude = 0.5 * fluid.density * rigidbody.speed_squared() * rigidbody.drag;
                let drag = -rigidbody.velocity.normalize_or_zero() * magnitude;
                rigidbody.apply_force(drag.truncate());
            }
        }
    }
}

/// Updates rigidbodies and applies transform changes
pub fn physics_after(time: Res<Time>, mut query: Query<(&mut Transform, &mut Rigidbody)>) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        rigidbody.update(&mut transform, time.delta_seconds());
    }
}
