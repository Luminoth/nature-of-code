//! Utilities

#![allow(dead_code)]

use bevy::prelude::*;

/// Gets the 2-argument arctangent of the vector
#[inline]
pub fn atan2(vec: Vec2) -> f32 {
    vec.x.atan2(vec.y)
}

/// Gets the heading angle of the vector
#[inline]
pub fn heading(vec: Vec2) -> f32 {
    atan2(vec)
}
