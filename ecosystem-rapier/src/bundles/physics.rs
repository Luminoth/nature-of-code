//! Physics bundles

use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

/// Physics object
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBodyBuilder,
    pub collider: ColliderBuilder,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PhysicsBundle {
    pub fn new_dynamic(position: Vec3, size: Vec2, mass: f32) -> Self {
        Self {
            rigidbody: RigidBodyBuilder::new_dynamic().additional_mass(mass),
            collider: ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0),
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn new_surface(position: Vec3, size: Vec2, friction: f32) -> Self {
        Self {
            rigidbody: RigidBodyBuilder::new_static(),
            collider: ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0).friction(friction),
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn new_fluid(position: Vec3, size: Vec2, density: f32) -> Self {
        Self {
            rigidbody: RigidBodyBuilder::new_static(),
            collider: ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0).density(density),
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }
}
