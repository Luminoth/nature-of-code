//! Creature components

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::resources::*;

//use super::debug::*;
use super::physics::*;

// NOTE: masses < 1 here can cause drag / friction to produce wildly oversized results

const FLY_COLOR: Color = Color::WHITE;
const FLY_MASS: f32 = 1.0; // completely made up mass to just make things work
const FLY_DRAG: f32 = 0.01;
const FLY_SIZE: f32 = 2.0;
pub const FLY_FORCE: f32 = FLY_MASS * 1500.0;

const FISH_COLOR: Color = Color::SALMON;
const FISH_MASS: f32 = 1500.0; // 100x the mass of an actual koi (kg)
const FISH_DRAG: f32 = 0.03;
const FISH_SIZE: f32 = 10.0;
const FISH_SWIM_DURATION: f32 = 3.0;
const FISH_COOLDOWN_DURATION: f32 = 3.0;
pub const FISH_FORCE: f32 = FISH_MASS * 300.0;

const SNAKE_COLOR: Color = Color::MAROON;
const SNAKE_MASS: f32 = 15.0; // 100x the mass of an actual garter snake (kg)
const SNAKE_DRAG: f32 = 0.04;
const SNAKE_SIZE: f32 = 5.0;
const SNAKE_DIRECTION_DURATION: f32 = 3.0;
pub const SNAKE_GROUND_FORCE: f32 = SNAKE_MASS * 200.0;

/// Shared creature component
#[derive(Default)]
pub struct Creature;

/// Flies fly
#[derive(Default)]
pub struct Fly;

impl Fly {
    /// Spawn a fly
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        id: u32,
        position: Vec2,
    ) {
        info!("spawning fly {} at {}", id, position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(FLY_SIZE, FLY_SIZE),
            ..Default::default()
        };

        let fly = Fly::default();

        let _entity = commands
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
            .insert(fly)
            .id();

        /*commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(30.0 + (15.0 * id as f32)),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "fly",
                TextStyle {
                    font: _asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(PhysicsDebug {
            name: format!("Fly {}", id),
            entity: _entity,
        });*/
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
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        random: &mut Random,
        _noise: &PerlinNoise,
        id: u32,
        position: Vec2,
    ) {
        info!("spawning fish {} at {}", id, position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(FISH_SIZE, FISH_SIZE),
            ..Default::default()
        };

        let mut fish = Fish::new(
            FISH_SWIM_DURATION + random.random_range(-1.0..1.0),
            FISH_COOLDOWN_DURATION + random.random_range(-1.0..1.0),
        );
        fish.swim_direction = random.direction();
        //fish.swim_direction = _noise.direction(random, 0.5);

        let _entity = commands
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
            .insert(fish)
            .id();

        /*commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(30.0 + (15.0 * id as f32)),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "fish",
                TextStyle {
                    font: _asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(PhysicsDebug {
            name: format!("Fish {}", id),
            entity: _entity,
        });*/
    }

    /// Construct a new fish that swims in a direction for the given duration
    pub fn new(swim_duration: f32, swim_cooldown: f32) -> Self {
        Self {
            swim_timer: Timer::from_seconds(swim_duration, false),
            swim_cooldown: Timer::from_seconds(swim_cooldown, false),
            ..Default::default()
        }
    }
}

/// Snakes snek
#[derive(Default)]
pub struct Snake {
    pub direction: Vec2,
    pub direction_timer: Timer,
}

impl Snake {
    /// Spawn a snake
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        random: &mut Random,
        _noise: &PerlinNoise,
        id: u32,
        position: Vec2,
    ) {
        info!("spawning snake {} at {}", id, position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(SNAKE_SIZE, SNAKE_SIZE),
            ..Default::default()
        };

        let mut snake = Snake::new(SNAKE_DIRECTION_DURATION + random.random_range(-0.5..0.5));
        snake.direction = random.direction();
        //snake.direction = _noise.direction(random, 0.5);

        let _entity = commands
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
            .insert(snake)
            .id();

        /*commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(30.0 + (15.0 * id as f32)),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "snake",
                TextStyle {
                    font: _asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(PhysicsDebug {
            name: format!("Snake {}", id),
            entity: _entity,
        });*/
    }

    /// Construct a new snake that slithers in a direction for the given duration
    pub fn new(direction_duration: f32) -> Self {
        Self {
            direction_timer: Timer::from_seconds(direction_duration, true),
            ..Default::default()
        }
    }
}
