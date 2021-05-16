//! Physics components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

/// Surface state
#[derive(Debug, Inspectable, Default)]
pub struct Surface {
    pub c: f32,
}

impl Surface {
    pub fn update(&self, rigidbody: &mut Rigidbody) {
        let magnitude = self.c;
        let direction = -rigidbody.velocity.normalize_or_zero();

        let friction = direction * magnitude;

        rigidbody.apply_force(friction.truncate());
    }
}

/// Fluid state
#[derive(Debug, Inspectable, Default)]
pub struct Fluid {
    pub density: f32,
}

impl Fluid {
    pub fn update(&self, rigidbody: &mut Rigidbody) {
        let speed_squared = rigidbody.speed_squared();
        let magnitude = 0.5 * self.density * speed_squared * rigidbody.drag;
        let direction = -rigidbody.velocity.normalize_or_zero();

        let drag = direction * magnitude;

        rigidbody.apply_force(drag.truncate());
    }
}

/// Oscillator
#[derive(Debug, Inspectable)]
pub struct Oscillator {
    pub angle: Vec2,
    pub velocity: Vec2,
    pub amplitude: Vec2,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            angle: Vec2::default(),
            velocity: Vec2::default(),
            amplitude: Vec2::splat(1.0),
        }
    }
}

impl Oscillator {
    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        self.angle += self.velocity * dt;

        transform.translation.x = self.angle.x.sin() * self.amplitude.x;
        transform.translation.y = self.angle.y.sin() * self.amplitude.y;
    }
}
