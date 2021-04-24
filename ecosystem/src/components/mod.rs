//! ECS components

pub mod creatures;

use bevy::prelude::*;

/// Physics state
#[derive(Default)]
pub struct Physics {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub topspeed: f64,
}
