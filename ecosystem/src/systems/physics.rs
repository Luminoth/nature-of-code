//! Physics systems

use bevy::prelude::*;

use crate::components::physics::*;

pub fn physics_collisions(
    mut query: Query<(&Transform, &mut Rigidbody, &Collider)>,
    surfaces: Query<(&Surface, &Transform, &Collider)>,
    fluids: Query<(&Fluid, &Transform, &Collider)>,
) {
    for (_transform, mut _rigidbody, _collider) in query.iter_mut() {
        // apply friction
        for (_surface, _stransform, _scollider) in surfaces.iter() {}

        // apply drag
        for (_fluid, _ftransform, _fcollider) in fluids.iter() {}
    }
}

pub fn physics_after(mut query: Query<(&mut Transform, &mut Rigidbody)>) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        rigidbody.update(&mut transform);
    }
}
