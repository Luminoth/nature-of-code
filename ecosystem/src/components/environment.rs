//! Environment components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::environment::*;
use crate::bundles::physics::*;

use super::physics::*;

// TODO: move all of these constants to the simulation params
// except maybe the colors

const AIR_DENSITY: f32 = 1.0;

const WATER_COLOR: Color = Color::rgba(0.18, 0.55, 0.34, 0.5);
const WATER_DENSITY: f32 = 1000.0;

const GROUND_COLOR: Color = Color::DARK_GREEN;
const GROUND_FRICTION: f32 = 15.0;

/// Ground
#[derive(Debug, Inspectable, Default)]
pub struct Ground;

impl Ground {
    /// Spawn a ground entity
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, i: usize, position: Vec2, size: Vec2) {
        commands
            .spawn_bundle(GroundBundle {
                surface: Surface { c: GROUND_FRICTION },
                physical: StaticPhysicsBundle {
                    collider: Collider {
                        size,
                        layer: CollisionLayer::Ground,
                    },
                    transform: Transform::from_translation(position.extend(10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Ground {}", i)))
            .with_children(|parent| {
                parent.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        width: size.x,
                        height: size.y,
                        origin: shapes::RectangleOrigin::Center,
                    },
                    ShapeColors::new(GROUND_COLOR),
                    DrawMode::Fill(FillOptions::default()),
                    Transform::default(),
                ));
            });
    }
}

/// Water
#[derive(Debug, Inspectable, Default)]
pub struct Water;

impl Water {
    /// Spawn a water entity
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, i: usize, position: Vec2, size: Vec2) {
        commands
            .spawn_bundle(WaterBundle {
                fluid: Fluid {
                    density: WATER_DENSITY,
                },
                physical: StaticPhysicsBundle {
                    collider: Collider {
                        size,
                        layer: CollisionLayer::Water,
                    },
                    transform: Transform::from_translation(position.extend(5.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Water {}", i)))
            .with_children(|parent| {
                parent.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        width: size.x,
                        height: size.y,
                        origin: shapes::RectangleOrigin::Center,
                    },
                    ShapeColors::new(WATER_COLOR),
                    DrawMode::Fill(FillOptions::default()),
                    Transform::default(),
                ));
            });
    }
}

/// Air
#[derive(Debug, Inspectable, Default)]
pub struct Air;

impl Air {
    /// Spawn an air entity
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, i: usize, size: Vec2) {
        commands
            .spawn_bundle(AirBundle {
                fluid: Fluid {
                    density: AIR_DENSITY,
                },
                physical: StaticPhysicsBundle {
                    collider: Collider {
                        size,
                        layer: CollisionLayer::Air,
                    },
                    transform: Transform::default(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Air {}", i)))
            .with_children(|parent| {
                parent.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle {
                        width: size.x,
                        height: size.y,
                        origin: shapes::RectangleOrigin::Center,
                    },
                    ShapeColors::new(WATER_COLOR),
                    DrawMode::Fill(FillOptions::default()),
                    Transform::default(),
                ));
            });
    }
}
