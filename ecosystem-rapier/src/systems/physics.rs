//! Physics systems

use bevy::prelude::*;

use crate::components::physics::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct Physics;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    Update,
}

/// Updates an oscillator
pub fn oscillator_update(time: Res<Time>, mut query: Query<(&mut Transform, &mut Oscillator)>) {
    let dt = time.delta_seconds();

    for (mut transform, mut oscillator) in query.iter_mut() {
        oscillator.update(dt, &mut transform);
    }
}
