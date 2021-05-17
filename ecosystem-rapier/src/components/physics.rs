//! Physics components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

#[derive(Debug, Default, Copy, Clone)]
struct Derivative {
    acceleration: Vec3,
    velocity: Vec3,
}

impl Derivative {
    fn evaluate(acceleration: Vec3, velocity: Vec3, dt: f32, derivative: Self) -> Self {
        Self {
            velocity: velocity + derivative.acceleration * dt,
            acceleration,
        }
    }
}

/// RK4 integration: https://gafferongames.com/post/integration_basics/
#[allow(dead_code)]
pub fn rk4_integrate(transform: &mut Transform, acceleration: Vec3, velocity: &mut Vec3, dt: f32) {
    // sample derivative at four points
    let a = Derivative::evaluate(acceleration, *velocity, 0.0, Derivative::default());
    let b = Derivative::evaluate(acceleration, *velocity, dt * 0.5, a);
    let c = Derivative::evaluate(acceleration, *velocity, dt * 0.5, b);
    let d = Derivative::evaluate(acceleration, *velocity, dt, c);

    // taylor expansion weighted sum
    let dvdt =
        1.0 / 6.0 * (a.acceleration + 2.0 * (b.acceleration + c.acceleration) + d.acceleration);
    let dxdt = 1.0 / 6.0 * (a.velocity + 2.0 * (b.velocity + c.velocity) + d.velocity);

    *velocity += dvdt * dt;
    transform.translation += dxdt * dt;
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
