//! Utilities

#![allow(dead_code)]

use bevy::prelude::*;

/// Gets the 2-argument arctangent of the vector
/// Useful for getting the "heading" angle of a vector
pub fn atan2(vec: Vec2) -> f32 {
    vec.x.atan2(vec.y)
}
