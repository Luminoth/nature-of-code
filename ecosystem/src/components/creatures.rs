//! Creature components

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

//use super::debug::*;
use super::physics::*;

// TODO: move all of these constants to the simulation params
// except maybe the colors

const FLY_COLOR: Color = Color::WHITE;
const FLY_MASS: f32 = 1.2; // 100000x the mass of an actual house fly (kg)
const FLY_DRAG: f32 = 0.01;
const FLY_SIZE: f32 = 1.0;
const FLY_REPEL_ACCEL: f32 = 1.0;
const FLY_ACCEL: f32 = 1500.0;

const FISH_COLOR: Color = Color::SALMON;
const FISH_MASS: f32 = 1500.0; // 100x the mass of an actual koi (kg)
const FISH_DRAG: f32 = 0.03;
const FISH_WIDTH: f32 = 10.0;
const FISH_LENGTH: f32 = 35.0;
const FISH_REPEL_ACCEL: f32 = 5.0;
const FISH_ACCEL: f32 = 300.0;

const SNAKE_COLOR: Color = Color::MAROON;
const SNAKE_MASS: f32 = 15.0; // 100x the mass of an actual garter snake (kg)
const SNAKE_DRAG: f32 = 0.04;
const SNAKE_WIDTH: f32 = 5.0;
const SNAKE_LENGTH: f32 = 60.0;
const SNAKE_REPEL_ACCEL: f32 = 10.0;
const SNAKE_GROUND_ACCEL: f32 = 400.0;
//const SNAKE_WATER_ACCEL: f32 = 300.0;

/// Shared creature component
#[derive(Default)]
pub struct Creature;

/// Flies fly
#[derive(Default)]
pub struct Fly {
    pub acceleration: f32,
    pub repel_acceleration: f32,
}

impl Fly {
    /// Spawn a fly
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        i: usize,
        position: Vec2,
    ) {
        info!("spawning fly {} at {}", i, position);

        let fly = Fly {
            acceleration: FLY_ACCEL,
            repel_acceleration: FLY_REPEL_ACCEL,
        };

        let rigidbody = Rigidbody {
            mass: FLY_MASS,
            drag: FLY_DRAG,
            ..Default::default()
        };

        let shape = shapes::Ellipse {
            radii: Vec2::new(FLY_SIZE, FLY_SIZE) * rigidbody.mass,
            ..Default::default()
        };

        let _entity = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(FLY_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(100.0)),
            ))
            .insert(rigidbody)
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
    pub acceleration: f32,
    pub repel_acceleration: f32,
}

impl Fish {
    /// Spawn a fish
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        i: usize,
        position: Vec2,
    ) {
        info!("spawning fish {} at {}", i, position);

        let fish = Fish {
            acceleration: FISH_ACCEL,
            repel_acceleration: FISH_REPEL_ACCEL,
        };

        let rigidbody = Rigidbody {
            mass: FISH_MASS,
            drag: FISH_DRAG,
            ..Default::default()
        };

        let shape = shapes::Rectangle {
            width: FISH_WIDTH * rigidbody.mass * 0.001,
            height: FISH_LENGTH * rigidbody.mass * 0.001,
            origin: shapes::RectangleOrigin::Center,
        };

        let _entity = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(FISH_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(0.0)),
            ))
            .insert(rigidbody)
            .insert(Collider::new(
                CollisionLayer::Water,
                shape.width,
                shape.height,
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
pub struct Snake {
    pub ground_acceleration: f32,
    pub repel_acceleration: f32,
}

impl Snake {
    /// Spawn a snake
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        _asset_server: &Res<AssetServer>,
        i: usize,
        position: Vec2,
    ) {
        info!("spawning snake {} at {}", i, position);

        let snake = Snake {
            ground_acceleration: SNAKE_GROUND_ACCEL,
            repel_acceleration: SNAKE_REPEL_ACCEL,
        };

        let rigidbody = Rigidbody {
            mass: SNAKE_MASS,
            drag: SNAKE_DRAG,
            ..Default::default()
        };

        let shape = shapes::Rectangle {
            width: SNAKE_WIDTH * rigidbody.mass * 0.1,
            height: SNAKE_LENGTH * rigidbody.mass * 0.1,
            origin: shapes::RectangleOrigin::Center,
        };

        let _entity = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(SNAKE_COLOR),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(position.extend(20.0)),
            ))
            .insert(rigidbody)
            .insert(Collider::new(
                CollisionLayer::Ground,
                shape.width,
                shape.height,
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
