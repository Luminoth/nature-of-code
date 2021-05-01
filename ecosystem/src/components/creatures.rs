//! Creature components

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

//use super::debug::*;
use super::physics::*;

// NOTE: masses < 1 here can cause drag / friction to produce wildly oversized results

const FLY_COLOR: Color = Color::WHITE;
const FLY_MASS: f32 = 1.2; // 100000x the mass of an actual house fly (kg)
const FLY_DRAG: f32 = 0.01;
const FLY_SIZE: f32 = 2.0;
pub const FLY_ACCEL: f32 = 1500.0;

const FISH_COLOR: Color = Color::SALMON;
const FISH_MASS: f32 = 1500.0; // 100x the mass of an actual koi (kg)
const FISH_DRAG: f32 = 0.03;
const FISH_SIZE: f32 = 10.0;
pub const FISH_ACCEL: f32 = 300.0;

const SNAKE_COLOR: Color = Color::MAROON;
const SNAKE_MASS: f32 = 15.0; // 100x the mass of an actual garter snake (kg)
const SNAKE_DRAG: f32 = 0.04;
const SNAKE_SIZE: f32 = 5.0;
pub const SNAKE_GROUND_ACCEL: f32 = 400.0;
//pub const SNAKE_WATER_ACCEL: f32 = 300.0;

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
pub struct Fish;

impl Fish {
    /// Spawn a fish
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        id: u32,
        position: Vec2,
    ) {
        info!("spawning fish {} at {}", id, position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(FISH_SIZE, FISH_SIZE),
            ..Default::default()
        };

        let fish = Fish::default();

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
}

/// Snakes snek
#[derive(Default)]
pub struct Snake;

impl Snake {
    /// Spawn a snake
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        id: u32,
        position: Vec2,
    ) {
        info!("spawning snake {} at {}", id, position);

        let shape = shapes::Ellipse {
            radii: Vec2::new(SNAKE_SIZE, SNAKE_SIZE),
            ..Default::default()
        };

        let snake = Snake::default();

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
}
