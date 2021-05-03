//! Creature components

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::creatures::*;
use crate::bundles::physics::*;

use super::physics::*;

// TODO: move all of these constants to the simulation params
// except maybe the colors

const FLY_COLOR: Color = Color::WHITE;
const FLY_MASS: f32 = 1.2; // 100000x the mass of an actual house fly (kg)
const FLY_DRAG: f32 = 0.01;
const FLY_SIZE: f32 = 2.0;
const FLY_REPEL_ACCEL: f32 = 1.0;
const FLY_ACCEL: f32 = 1500.0;

const FISH_BODY_COLOR: Color = Color::SILVER;
const FISH_HEAD_COLOR: Color = Color::SALMON;
const FISH_MASS: f32 = 1500.0; // 100x the mass of an actual koi (kg)
const FISH_DRAG: f32 = 0.03;
const FISH_WIDTH: f32 = 10.0;
const FISH_LENGTH: f32 = 30.0;
const FISH_REPEL_ACCEL: f32 = 5.0;
const FISH_ACCEL: f32 = 300.0;

const SNAKE_BODY_COLOR: Color = Color::MAROON;
const SNAKE_HEAD_COLOR: Color = Color::ORANGE_RED;
const SNAKE_MASS: f32 = 15.0; // 100x the mass of an actual garter snake (kg)
const SNAKE_DRAG: f32 = 0.04;
const SNAKE_WIDTH: f32 = 5.0;
const SNAKE_LENGTH: f32 = 40.0;
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

        let mass = FLY_MASS;
        let size = Vec2::new(FLY_SIZE, FLY_SIZE) * mass;

        commands
            .spawn_bundle(FlyBundle {
                fly: Fly {
                    acceleration: FLY_ACCEL,
                    repel_acceleration: FLY_REPEL_ACCEL,
                },
                physical: PhysicalBundle {
                    rigidbody: Rigidbody {
                        mass,
                        drag: FLY_DRAG,
                        ..Default::default()
                    },
                    collider: Collider {
                        size,
                        layer: CollisionLayer::Air,
                    },
                    transform: Transform::from_translation(position.extend(0.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Ellipse {
                        radii: size * 0.5,
                        ..Default::default()
                    },
                    ShapeColors::new(FLY_COLOR),
                    DrawMode::Fill(FillOptions::default()),
                    Transform::default(),
                ));
            });
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

        let mass = FISH_MASS;
        let size = Vec2::new(FISH_WIDTH, FISH_LENGTH) * mass * 0.001;
        let head_size = Vec2::new(size.x * 0.5, size.y * 0.25);

        commands
            .spawn_bundle(FishBundle {
                fish: Fish {
                    acceleration: FISH_ACCEL,
                    repel_acceleration: FISH_REPEL_ACCEL,
                },
                physical: PhysicalBundle {
                    rigidbody: Rigidbody {
                        mass,
                        drag: FISH_DRAG,
                        ..Default::default()
                    },
                    collider: Collider {
                        size,
                        layer: CollisionLayer::Water,
                    },
                    transform: Transform::from_translation(position.extend(0.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Ellipse {
                            radii: size * 0.5,
                            ..Default::default()
                        },
                        ShapeColors::new(FISH_BODY_COLOR),
                        DrawMode::Fill(FillOptions::default()),
                        Transform::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_bundle(GeometryBuilder::build_as(
                            &shapes::Ellipse {
                                radii: head_size * 0.5,
                                ..Default::default()
                            },
                            ShapeColors::new(FISH_HEAD_COLOR),
                            DrawMode::Fill(FillOptions::default()),
                            Transform::from_translation(Vec3::new(
                                0.0,
                                size.y * 0.5 - head_size.y * 0.5,
                                1.0,
                            )),
                        ));
                    });
            });
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

        let mass = SNAKE_MASS;
        let size = Vec2::new(SNAKE_WIDTH, SNAKE_LENGTH) * mass * 0.1;
        let head_size = Vec2::splat(size.x * 0.5);

        commands
            .spawn_bundle(SnakeBundle {
                snake: Snake {
                    ground_acceleration: SNAKE_GROUND_ACCEL,
                    repel_acceleration: SNAKE_REPEL_ACCEL,
                },
                physical: PhysicalBundle {
                    rigidbody: Rigidbody {
                        mass,
                        drag: SNAKE_DRAG,
                        ..Default::default()
                    },
                    collider: Collider {
                        size,
                        layer: CollisionLayer::Ground,
                    },
                    transform: Transform::from_translation(position.extend(20.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Ellipse {
                            radii: size * 0.5,
                            ..Default::default()
                        },
                        ShapeColors::new(SNAKE_BODY_COLOR),
                        DrawMode::Fill(FillOptions::default()),
                        Transform::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_bundle(GeometryBuilder::build_as(
                            &shapes::Ellipse {
                                radii: head_size * 0.5,
                                ..Default::default()
                            },
                            ShapeColors::new(SNAKE_HEAD_COLOR),
                            DrawMode::Fill(FillOptions::default()),
                            Transform::from_translation(Vec3::new(
                                0.0,
                                size.y * 0.5 - head_size.y * 0.5,
                                1.0,
                            )),
                        ));
                    });
            });
    }
}
