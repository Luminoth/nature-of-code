//! Environment systems

use bevy::prelude::*;

use crate::components::environment::*;

/// Environment systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum EnvironmentsSystem {
    Physics,
}

/// Water current
pub fn water_current(_time: Res<Time>, mut _query: Query<&mut WaterCurrent, With<Water>>) {
    // TODO: apply force to creatures contained in the associated environment
    // then update the force vector (or do that first?)
}

/// Wind
pub fn wind(_time: Res<Time>, mut _query: Query<&mut Wind, With<Air>>) {
    // TODO: apply force to creatures contained in the associated environment
    // then update the force vector (or do that first?)
}
