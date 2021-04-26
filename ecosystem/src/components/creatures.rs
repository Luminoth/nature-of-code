//! Creature components

use bevy::prelude::*;

/// Shared creature component
#[derive(Default)]
pub struct Creature;

/// Flies fly
#[derive(Default)]
pub struct Fly;

/// Fish swim
#[derive(Default)]
pub struct Fish {
    pub swim_direction: Vec2,
    pub swim_timer: Timer,
    pub swim_cooldown: Timer,
}

impl Fish {
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
    /// Construct a new snake that slithers in a direction for the given duration
    pub fn new(direction_duration: f32) -> Self {
        Self {
            direction_timer: Timer::from_seconds(direction_duration, false),
        }
    }
}
