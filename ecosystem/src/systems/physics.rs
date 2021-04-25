//! Physics systems

use bevy::prelude::*;

use crate::components::physics::*;

pub fn physics_collisions(
    mut query: Query<(&Transform, &mut Rigidbody, &Collider)>,
    surfaces: Query<(&Surface, &Transform, &Collider)>,
    fluids: Query<(&Fluid, &Transform, &Collider)>,
) {
    for (transform, mut rigidbody, collider) in query.iter_mut() {
        // apply friction
        for (surface, stransform, scollider) in surfaces.iter() {
            if collider.collides(transform, scollider, stransform) {
                let friction = (rigidbody.velocity * -1.0).normalize() * surface.c;
                rigidbody.apply_force(friction.truncate());
            }
        }

        // apply drag
        for (fluid, ftransform, fcollider) in fluids.iter() {
            if collider.collides(transform, fcollider, ftransform) {
                let drag_magnitude = fluid.c * rigidbody.speed_squared();
                let drag = (rigidbody.velocity * -1.0).normalize() * drag_magnitude;
                rigidbody.apply_force(drag.truncate());
            }
        }
    }
}

pub fn physics_after(mut query: Query<(&mut Transform, &mut Rigidbody)>) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        rigidbody.update(&mut transform);
    }
}
