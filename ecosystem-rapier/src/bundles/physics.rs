//! Physics bundles

use bevy::prelude::*;

use crate::components::physics::*;

/// Static object
#[derive(Default, Bundle)]
pub struct StaticPhysicsBundle {
    pub collider: Collider,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Dynamic object
#[derive(Default, Bundle)]
pub struct DynamicPhysicsBundle {
    pub rigidbody: Rigidbody,
    pub collider: Collider,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
