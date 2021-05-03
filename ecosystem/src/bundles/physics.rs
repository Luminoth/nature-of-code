//! Physics bundles

use bevy::prelude::*;

use crate::components::physics::*;

/// Basic physical
#[derive(Default, Bundle)]
pub struct PhysicalBundle {
    pub rigidbody: Rigidbody,
    pub collider: Collider,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
