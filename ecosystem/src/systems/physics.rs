//! Physics systems

use bevy::prelude::*;

use crate::components::physics::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct Physics;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    Collisions,
    Update,
    Contain,
    Repel,
    Debug,
}

fn window_bounds(hw: f32, hh: f32, offset: f32, collider: &Collider) -> (f32, f32, f32, f32) {
    let minx = -hw + collider.size.x + offset;
    let maxx = hw - collider.size.x - offset;
    let miny = -hh + collider.size.y + offset;
    let maxy = hh - collider.size.y - offset;

    (minx, maxx, miny, maxy)
}

/// Repel bodies from the window border
pub fn window_repel(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider)>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    let offset = 5.0;

    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        let (minx, maxx, miny, maxy) = window_bounds(hw, hh, offset, collider);
        rigidbody.repel(&mut transform, minx, maxx, miny, maxy);
    }
}

/// Contains bodies inside the window
pub fn window_contain(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Rigidbody, &Collider)>,
) {
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    let offset = 5.0;

    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        let (minx, maxx, miny, maxy) = window_bounds(hw, hh, offset, collider);
        rigidbody.contain(&mut transform, minx, maxx, miny, maxy);
    }
}

/// Handles physics collisions
// TODO: this name sucks
pub fn physics_collisions(
    mut query: Query<(&Transform, &mut Rigidbody, &Collider)>,
    surfaces: Query<(&Surface, &Transform, &Collider)>,
    fluids: Query<(&Fluid, &Transform, &Collider)>,
) {
    for (transform, mut rigidbody, collider) in query.iter_mut() {
        for (surface, stransform, scollider) in surfaces.iter() {
            if collider.collides(transform, scollider, stransform) {
                surface.update(&mut rigidbody);
            }
        }

        for (fluid, ftransform, fcollider) in fluids.iter() {
            if collider.collides(transform, fcollider, ftransform) {
                fluid.update(&mut rigidbody);
            }
        }
    }
}

/// Updates rigidbodies and applies transform changes
pub fn physics_update(mut query: Query<(&mut Transform, &mut Rigidbody)>) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        rigidbody.update(&mut transform);
    }
}
