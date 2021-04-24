//! Creature systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;

use crate::components::creatures::*;

pub fn fly(
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<&RigidBodyHandleComponent, With<Fly>>,
) {
    for rbhandle in query.iter_mut() {
        if let Some(_rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {}
    }
}

pub fn fish(
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<&RigidBodyHandleComponent, With<Fish>>,
) {
    for rbhandle in query.iter_mut() {
        if let Some(_rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {}
    }
}

pub fn snake(
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<&RigidBodyHandleComponent, With<Snake>>,
) {
    for rbhandle in query.iter_mut() {
        if let Some(_rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {}
    }
}
