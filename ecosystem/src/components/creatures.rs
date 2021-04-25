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
    pub timer: Timer,
}

impl Fish {
    /// Construct a new fish that swims in a direction for the given duration
    pub fn new(direction_duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(direction_duration, true),
        }
    }
}

/// Snakes snek
#[derive(Default)]
pub struct Snake {
    pub timer: Timer,
}

impl Snake {
    /// Construct a new snake that slithers in a direction for the given duration
    pub fn new(direction_duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(direction_duration, true),
        }
    }
}
