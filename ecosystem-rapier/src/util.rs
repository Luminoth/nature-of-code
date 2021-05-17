//! Utilities

#![allow(dead_code)]

use bevy::prelude::*;
use bevy_rapier2d::rapier::math::{Real, Vector};

/// Gets the 2-argument arctangent of the vector
/// Useful for getting the "heading" angle of a vector
pub fn atan2(vec: Vec2) -> f32 {
    vec.x.atan2(vec.y)
}

/// Convert a glam Vec2 to an nalgebra Vector<Real>
///
/// TODO: this can go away once bevy_rapier2d exposes the From trait for this
pub fn to_vector(v: Vec2) -> Vector<Real> {
    (*v.as_ref()).into()
}

/// Convert an nalgebra Vector<Real> to a glam Vec2
///
/// TODO: this can go away once bevy_rapier2d exposes the From trait for this
pub fn from_vector(v: Vector<Real>) -> Vec2 {
    Vec2::new(v[0], v[1])
}
