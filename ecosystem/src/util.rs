//! Utilities

#![allow(dead_code)]

use bevy::prelude::*;

pub fn atan2(vec: Vec2) -> f32 {
    vec.x.atan2(vec.y)
}
