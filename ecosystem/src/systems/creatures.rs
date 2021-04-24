//! Creature systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;

use crate::components::creatures::*;

pub fn fly(mut query: Query<&RigidBodyHandleComponent, With<Fly>>) {
    for _rbhandle in query.iter_mut() {}
}

pub fn fish(mut query: Query<&RigidBodyHandleComponent, With<Fish>>) {
    for _rbhandle in query.iter_mut() {}
}

pub fn snake(mut query: Query<&RigidBodyHandleComponent, With<Snake>>) {
    for _rbhandle in query.iter_mut() {}
}
