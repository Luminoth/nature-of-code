//! Environment components

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::physics::*;

const AIR_DENSITY: f32 = 1.0;

const WATER_COLOR: Color = Color::rgba(0.18, 0.55, 0.34, 0.5);
const WATER_DENSITY: f32 = 1000.0;

// TODO: temporarily transparent because fish can swim under it
const GROUND_COLOR: Color = Color::rgba(0.0, 0.5, 0.0, 0.5); //Color::DARK_GREEN;
const GROUND_FRICTION: f32 = 0.5;

/// Ground
#[derive(Default)]
pub struct Ground;

impl Ground {
    /// Spawn a ground entity
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, position: Vec2, size: Vec2) {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Rectangle {
                    width: size.x,
                    height: size.y,
                    origin: shapes::RectangleOrigin::TopLeft,
                },
                ShapeColors::new(GROUND_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(10.0)),
            ))
            .insert(Surface::new(GROUND_FRICTION))
            .insert(Collider::new(CollisionLayer::Ground, size.x, size.y))
            .insert(Ground::default());
    }
}

/// Water
#[derive(Default)]
pub struct Water;

impl Water {
    /// Spawn a water entity
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, position: Vec2, size: Vec2) {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Rectangle {
                    width: size.x,
                    height: size.y,
                    origin: shapes::RectangleOrigin::TopLeft,
                },
                ShapeColors::new(WATER_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(5.0)),
            ))
            .insert(Fluid::new(WATER_DENSITY))
            .insert(Collider::new(CollisionLayer::Water, size.x, size.y))
            .insert(Water::default());
    }
}

/// Air
#[derive(Default)]
pub struct Air;

impl Air {
    /// Spawn an air entity
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, size: Vec2) {
        commands
            .spawn()
            .insert(Transform::default())
            .insert(Fluid::new(AIR_DENSITY))
            .insert(Collider::new(CollisionLayer::Air, size.x, size.y))
            .insert(Air::default());
    }
}
