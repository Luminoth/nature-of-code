//! Creature components

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::physics::*;

const FLY_COLOR: Color = Color::WHITE;
const FLY_MASS: f32 = 0.1;
const FLY_DRAG: f32 = 0.1;
const FLY_SIZE: f32 = 2.0;
pub const FLY_FORCE: f32 = FLY_MASS * 200.0;

const FISH_COLOR: Color = Color::SALMON;
const FISH_MASS: f32 = 15.0;
const FISH_DRAG: f32 = 0.1;
const FISH_SIZE: f32 = 10.0;
pub const FISH_FORCE: f32 = FISH_MASS * 5000.0;

const SNAKE_COLOR: Color = Color::MAROON;
const SNAKE_MASS: f32 = 0.15;
const SNAKE_DRAG: f32 = 0.2;
const SNAKE_SIZE: f32 = 5.0;
pub const SNAKE_GROUND_FORCE: f32 = SNAKE_MASS * 7000.0;

/// Shared creature component
#[derive(Default)]
pub struct Creature;

/// Flies fly
#[derive(Default)]
pub struct Fly;

impl Fly {
    /// Spawn a fly
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, position: Vec2) {
        info!("spawning fly at {}", position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(FLY_SIZE, FLY_SIZE),
            ..Default::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(FLY_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(100.0)),
            ))
            .insert(Rigidbody {
                mass: FLY_MASS,
                drag: FLY_DRAG,
                ..Default::default()
            })
            .insert(Collider::new(
                CollisionLayer::Air,
                shape.radii.x * 2.0,
                shape.radii.y * 2.0,
            ))
            .insert(Creature::default())
            .insert(Fly::default());
    }
}

/// Fish swim
#[derive(Default)]
pub struct Fish {
    pub swim_direction: Vec2,
    pub swim_timer: Timer,
    pub swim_cooldown: Timer,
}

impl Fish {
    /// Spawn a fish
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, position: Vec2) {
        info!("spawning fish at {}", position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(FISH_SIZE, FISH_SIZE),
            ..Default::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(FISH_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(0.0)),
            ))
            .insert(Rigidbody {
                mass: FISH_MASS,
                drag: FISH_DRAG,
                ..Default::default()
            })
            .insert(Collider::new(
                CollisionLayer::Water,
                shape.radii.x * 2.0,
                shape.radii.y * 2.0,
            ))
            .insert(Creature::default())
            .insert(Fish::new(2.0, 2.0));
    }

    /// Construct a new fish that swims in a direction for the given duration
    pub fn new(swim_duration: f32, swim_cooldown: f32) -> Self {
        Self {
            swim_direction: Vec2::default(),
            swim_timer: Timer::from_seconds(swim_duration, false),
            swim_cooldown: Timer::from_seconds(swim_cooldown, false),
        }
    }
}

/// Snakes snek
#[derive(Default)]
pub struct Snake {
    pub direction_timer: Timer,
}

impl Snake {
    /// Spawn a snake
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, position: Vec2) {
        info!("spawning snake at {}", position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(SNAKE_SIZE, SNAKE_SIZE),
            ..Default::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(SNAKE_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(20.0)),
            ))
            .insert(Rigidbody {
                mass: SNAKE_MASS,
                drag: SNAKE_DRAG,
                ..Default::default()
            })
            .insert(Collider::new(
                CollisionLayer::Ground,
                shape.radii.x * 2.0,
                shape.radii.y * 2.0,
            ))
            .insert(Creature::default())
            .insert(Snake::new(2.0));
    }

    /// Construct a new snake that slithers in a direction for the given duration
    pub fn new(direction_duration: f32) -> Self {
        Self {
            direction_timer: Timer::from_seconds(direction_duration, true),
        }
    }
}
