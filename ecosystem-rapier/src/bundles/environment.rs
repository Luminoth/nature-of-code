//! Environment bundles

use bevy::prelude::*;

use crate::components::environment::*;
use crate::components::physics::*;

use super::physics::*;

/// Ground bundle
#[derive(Default, Bundle)]
pub struct GroundBundle {
    pub ground: Ground,
    pub surface: Surface,

    #[bundle]
    pub physical: StaticPhysicsBundle,
}

/// Water bundle
#[derive(Default, Bundle)]
pub struct WaterBundle {
    pub water: Water,
    pub fluid: Fluid,
    pub current: WaterCurrent,

    #[bundle]
    pub physical: StaticPhysicsBundle,
}

/// Air bundle
#[derive(Default, Bundle)]
pub struct AirBundle {
    pub air: Air,
    pub fluid: Fluid,
    pub wind: Wind,

    #[bundle]
    pub physical: StaticPhysicsBundle,
}
