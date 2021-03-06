//! Creature components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::creatures::*;
use crate::bundles::physics::*;
use crate::resources::*;

use super::particles::*;
use super::physics::*;

// TODO: move all of these constants to the simulation params
// except maybe the colors

// fly is much larger than an actual fly
// so that they're actually visible
const FLY_COLOR: Color = Color::WHITE;
pub const FIREFLY_COLOR: Color = Color::YELLOW_GREEN;
const FLY_MASS: f32 = 1.0;
const FLY_DRAG: f32 = 0.01;
pub const FLY_SIZE: f32 = 0.05 / FLY_MASS;
const FLY_REPEL_ACCEL: f32 = 0.01;
const FLY_ACCEL: f32 = 5.0;

const FISH_BODY_COLOR: Color = Color::SILVER;
const FISH_HEAD_COLOR: Color = Color::SALMON;
const FISH_MASS: f32 = 15.0;
const FISH_DRAG: f32 = 0.03;
pub const FISH_WIDTH: f32 = 0.3 / FISH_MASS;
pub const FISH_LENGTH: f32 = 0.6 / FISH_MASS;
const FISH_REPEL_ACCEL: f32 = 0.01;
const FISH_ACCEL: f32 = 1.0;

const SNAKE_BODY_COLOR: Color = Color::MAROON;
const SNAKE_HEAD_COLOR: Color = Color::ORANGE_RED;
const SNAKE_MASS: f32 = 2.0;
const SNAKE_DRAG: f32 = 0.04;
pub const SNAKE_WIDTH: f32 = 0.1 / SNAKE_MASS;
pub const SNAKE_LENGTH: f32 = 0.8 / SNAKE_MASS;
const SNAKE_REPEL_ACCEL: f32 = 0.01;
const SNAKE_GROUND_ACCEL: f32 = 8.0;
//const SNAKE_WATER_ACCEL: f32 = 1.0;

/// Shared creature component
#[derive(Debug, Default, Component, Inspectable)]
pub struct Creature {
    #[inspectable(read_only)]
    pub acceleration_direction: Vec2,
}

/// Flies fly
#[derive(Debug, Default, Component, Inspectable)]
pub struct Fly {
    pub acceleration: f32,
    pub repel_acceleration: f32,
}

impl Fly {
    fn firefly_particles(random: &mut Random, color: Color) -> ParticleSystem {
        // TODO: we can calculate the required capacity
        // from the spawn rate and lifespan
        let mut particles = ParticleSystem::with_capacity("Firefly", color, 20);
        particles.spawn_rate = 0.05;
        particles.particle_lifespan = 0.5;
        particles.max_speed = random.normal(0.5, 0.1);
        particles.size = Vec2::splat(0.05);

        particles
    }

    /// Spawn a fly
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        random: &mut Random,
        i: usize,
        position: Vec2,
        color: Color,
    ) {
        let is_firefly = random.coin();
        if is_firefly {
            info!("spawning firefly {} at {}", i, position);
        } else {
            info!("spawning fly {} at {}", i, position);
        }

        let mass = FLY_MASS; // TODO: modifier
        let size = Vec2::new(FLY_SIZE, FLY_SIZE) * mass;

        let mut bundle = commands.spawn_bundle(FlyBundle {
            fly: Fly {
                acceleration: FLY_ACCEL,
                repel_acceleration: FLY_REPEL_ACCEL,
            },
            physical: DynamicPhysicsBundle {
                rigidbody: Rigidbody {
                    mass,
                    drag: FLY_DRAG,
                    ..Default::default()
                },
                collider: Collider::Box(
                    BoxCollider::new(Vec2::default(), size),
                    CollisionLayer::Air,
                ),
                transform: Transform::from_translation(position.extend(40.0)),
                ..Default::default()
            },
            ..Default::default()
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Ellipse {
                        radii: size * 0.5,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: if is_firefly { FIREFLY_COLOR } else { FLY_COLOR },
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"))
                .insert(Oscillator {
                    angle: Vec2::new(random.random_range(0.0..2.0 * std::f32::consts::PI), 0.0),
                    velocity: Vec2::new(10.0, 0.0),
                    amplitude: Vec2::new(0.1, 0.0),
                });
        });

        if is_firefly {
            bundle
                .insert(Name::new(format!("Firefly {}", i)))
                .with_children(|parent| {
                    parent.spawn_bundle(FireflyBundle {
                        particles: Self::firefly_particles(random, color),
                        ..Default::default()
                    });
                });
        } else {
            bundle.insert(Name::new(format!("Fly {}", i)));
        }
    }
}

/// Fireflies fly... and glow
#[derive(Debug, Default, Component, Inspectable)]
pub struct Firefly;

/// Fish swim
#[derive(Debug, Default, Component, Inspectable)]
pub struct Fish {
    pub acceleration: f32,
    pub repel_acceleration: f32,
}

impl Fish {
    pub fn particles(random: &mut Random, color: Color) -> ParticleSystem {
        // TODO: we can calculate the required capacity
        // from the spawn rate and lifespan
        let mut particles = ParticleSystem::with_capacity("Fish", color, 20);
        particles.spawn_rate = 0.05;
        particles.particle_lifespan = 0.5;
        particles.max_speed = random.normal(0.3, 0.1);

        particles
    }

    /// Spawn a fish
    #[allow(dead_code)]
    pub fn spawn(
        commands: &mut Commands,
        random: &mut Random,
        i: usize,
        position: Vec2,
        color: Color,
    ) {
        info!("spawning fish {} at {}", i, position);

        let mass = FISH_MASS; // TODO: modifier
        let size = Vec2::new(FISH_WIDTH, FISH_LENGTH) * mass;
        let head_size = Vec2::new(size.x * 0.5, size.y * 0.25);

        commands
            .spawn_bundle(FishBundle {
                fish: Fish {
                    acceleration: FISH_ACCEL,
                    repel_acceleration: FISH_REPEL_ACCEL,
                },
                physical: DynamicPhysicsBundle {
                    rigidbody: Rigidbody {
                        mass,
                        drag: FISH_DRAG,
                        ..Default::default()
                    },
                    collider: Collider::Box(
                        BoxCollider::new(Vec2::default(), size),
                        CollisionLayer::Water,
                    ),
                    transform: Transform::from_translation(position.extend(0.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Fish {}", i)))
            .with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Ellipse {
                            radii: size * 0.5,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: FISH_BODY_COLOR,
                            options: FillOptions::default(),
                        }),
                        Transform::default(),
                    ))
                    .insert(Name::new("Model"))
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(GeometryBuilder::build_as(
                                &shapes::Ellipse {
                                    radii: head_size * 0.5,
                                    ..Default::default()
                                },
                                DrawMode::Fill(FillMode {
                                    color: FISH_HEAD_COLOR,
                                    options: FillOptions::default(),
                                }),
                                Transform::from_translation(Vec3::new(
                                    0.0,
                                    size.y * 0.5 - head_size.y * 0.5,
                                    1.0,
                                )),
                            ))
                            .insert(Name::new("Head"));
                    })
                    .insert(Oscillator {
                        angle: Vec2::new(random.random_range(0.0..2.0 * std::f32::consts::PI), 0.0),
                        velocity: Vec2::new(20.0, 0.0),
                        amplitude: Vec2::new(0.1, 0.0),
                    });

                parent.spawn_bundle(FishParticlesBundle {
                    particles: Self::particles(random, color),
                    transform: Transform::from_translation(Vec3::new(0.0, -size.y * 0.5, 1.0)),
                    ..Default::default()
                });
            });
    }
}

/// Snakes snek
#[derive(Debug, Default, Component, Inspectable)]
pub struct Snake {
    pub ground_acceleration: f32,
    pub repel_acceleration: f32,
}

impl Snake {
    /// Spawn a snake
    #[allow(dead_code)]
    pub fn spawn(commands: &mut Commands, random: &mut Random, i: usize, position: Vec2) {
        info!("spawning snake {} at {}", i, position);

        let mass = SNAKE_MASS; // TODO: modifier
        let size = Vec2::new(SNAKE_WIDTH, SNAKE_LENGTH) * mass;
        let head_size = Vec2::splat(size.x * 0.5);

        commands
            .spawn_bundle(SnakeBundle {
                snake: Snake {
                    ground_acceleration: SNAKE_GROUND_ACCEL,
                    repel_acceleration: SNAKE_REPEL_ACCEL,
                },
                physical: DynamicPhysicsBundle {
                    rigidbody: Rigidbody {
                        mass,
                        drag: SNAKE_DRAG,
                        ..Default::default()
                    },
                    collider: Collider::Box(
                        BoxCollider::new(Vec2::default(), size),
                        CollisionLayer::Ground,
                    ),
                    transform: Transform::from_translation(position.extend(20.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Snake {}", i)))
            .with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Ellipse {
                            radii: size * 0.5,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: SNAKE_BODY_COLOR,
                            options: FillOptions::default(),
                        }),
                        Transform::default(),
                    ))
                    .insert(Name::new("Model"))
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(GeometryBuilder::build_as(
                                &shapes::Ellipse {
                                    radii: head_size * 0.5,
                                    ..Default::default()
                                },
                                DrawMode::Fill(FillMode {
                                    color: SNAKE_HEAD_COLOR,
                                    options: FillOptions::default(),
                                }),
                                Transform::from_translation(Vec3::new(
                                    0.0,
                                    size.y * 0.5 - head_size.y * 0.5,
                                    1.0,
                                )),
                            ))
                            .insert(Name::new("Head"));
                    })
                    .insert(Oscillator {
                        angle: Vec2::new(random.random_range(0.0..2.0 * std::f32::consts::PI), 0.0),
                        velocity: Vec2::new(30.0, 0.0),
                        amplitude: Vec2::new(0.1, 0.0),
                    });
            });
    }
}
