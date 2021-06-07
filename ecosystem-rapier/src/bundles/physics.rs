//! Physics bundles

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::physics::*;

/// Physics object
#[derive(Bundle)]
pub struct PhysicsBundle {
    #[bundle]
    pub rigidbody: RigidBodyBundle,
    #[bundle]
    pub collider: ColliderBundle,

    pub physical: Physical,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PhysicsBundle {
    pub fn new_dynamic(position: Vec3, size: Vec2, mass: f32) -> Self {
        Self {
            rigidbody: RigidBodyBundle {
                position: position.into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0),
                ..Default::default()
            },
            physical: Physical {
                previous_position: position,
            },
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn new_surface(position: Vec3, size: Vec2, friction: f32) -> Self {
        Self {
            rigidbody: RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: position.into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0),
                collider_type: ColliderType::Sensor,
                material: ColliderMaterial {
                    friction,
                    ..Default::default()
                },
                ..Default::default()
            },
            physical: Physical {
                previous_position: position,
            },
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }

    pub fn new_fluid(position: Vec3, size: Vec2, density: f32) -> Self {
        Self {
            rigidbody: RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: position.into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0),
                collider_type: ColliderType::Sensor,
                mass_properties: ColliderMassProps::Density(density),
                ..Default::default()
            },
            physical: Physical {
                previous_position: position,
            },
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }
}
