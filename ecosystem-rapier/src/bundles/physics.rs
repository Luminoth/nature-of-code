//! Physics bundles

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::physics::*;

/// Physics object
#[derive(Bundle)]
pub struct PhysicsBundle {
    #[bundle]
    pub rigidbody: RigidBodyBundle,
    pub rbsync: RigidBodyPositionSync,

    #[bundle]
    pub collider: ColliderBundle,

    pub physical: Physical,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PhysicsBundle {
    pub fn new_dynamic(position: Vec3, size: Vec2, mass: f32, drag: f32) -> Self {
        Self {
            rigidbody: RigidBodyBundle {
                position: position.into(),
                mass_properties: RigidBodyMassProps {
                    flags: RigidBodyMassPropsFlags::ROTATION_LOCKED_X
                        | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y,
                    local_mprops: MassProperties {
                        inv_mass: 1.0 / mass,
                        inv_principal_inertia_sqrt: 0.0,
                        local_com: Point::origin(),
                    },
                    ..Default::default()
                }
                .into(),
                damping: RigidBodyDamping {
                    linear_damping: drag,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
            rbsync: RigidBodyPositionSync::Discrete,
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0).into(),
                mass_properties: ColliderMassProps::Density(0.0).into(),
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
                body_type: RigidBodyType::Static.into(),
                position: position.into(),
                mass_properties: RigidBodyMassProps {
                    flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
            rbsync: RigidBodyPositionSync::Discrete,
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0).into(),
                collider_type: ColliderType::Sensor.into(),
                material: ColliderMaterial {
                    friction,
                    ..Default::default()
                }
                .into(),
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
                body_type: RigidBodyType::Static.into(),
                position: position.into(),
                mass_properties: RigidBodyMassProps {
                    flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
            rbsync: RigidBodyPositionSync::Discrete,
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0).into(),
                collider_type: ColliderType::Sensor.into(),
                mass_properties: ColliderMassProps::Density(density).into(),
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
