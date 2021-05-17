//! Environment bundles

use bevy::prelude::*;

use crate::components::environment::*;

use super::physics::*;

/// Ground bundle
#[derive(Bundle)]
pub struct GroundBundle {
    pub ground: Ground,

    #[bundle]
    pub physical: PhysicsBundle,
}

/// Water bundle
#[derive(Bundle)]
pub struct WaterBundle {
    pub water: Water,
    pub current: WaterCurrent,

    #[bundle]
    pub physical: PhysicsBundle,
}

/// Air bundle
#[derive(Bundle)]
pub struct AirBundle {
    pub air: Air,
    pub wind: Wind,

    #[bundle]
    pub physical: PhysicsBundle,
}
