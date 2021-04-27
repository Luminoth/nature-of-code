//! Debug components

use bevy::prelude::*;

/// Marker for FPS UI text
#[derive(Default)]
pub struct FpsText;

/// Marker for Physics Debug text
#[derive(Debug)]
pub struct PhysicsDebug {
    pub name: String,
    pub entity: Entity,
}
