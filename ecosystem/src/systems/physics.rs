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
                let direction = -rigidbody.velocity.normalize_or_zero();

                let friction = direction * magnitude;
                if !friction.is_finite() {
                    panic!(
                        "Invalid friction c: {}, direction: {}",
                        surface.c, direction
                    );
                }

                rigidbody.apply_force(friction.truncate());
            }
        }

        // apply drag
        for (fluid, ftransform, fcollider) in fluids.iter() {
            if collider.collides(transform, fcollider, ftransform) {
                let speed_squared = rigidbody.speed_squared();
                let magnitude = 0.5 * fluid.density * speed_squared * rigidbody.drag;
                let direction = -rigidbody.velocity.normalize_or_zero();

                let drag = direction * magnitude;
                if !drag.is_finite() {
                    panic!(
                        "Invalid drag p: {}, v2: {}, c: {}, direction: {}",
                        fluid.density, speed_squared, rigidbody.drag, direction
                    );
                }

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
