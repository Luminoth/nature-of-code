//! Physics components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use num_traits::Float;

use crate::util::to_vector;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

/// Oscillator
#[derive(Debug, Inspectable)]
pub struct Physical {
    #[inspectable(ignore)]
    pub previous_position: Vec3,
}

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

/// Contain a rigidbody inside bounds
#[allow(dead_code)]
pub fn contain(
    rigidbody: &mut RigidBody,
    transform: &mut Transform,
    physical: &Physical,
    min: Vec2,
    max: Vec2,
    min_distance: f32,
) {
    // unwind to our previous position, if we can
    // otherwise clamp to the min / max minus a little fudge

    if transform.translation.x <= min.x {
        transform.translation.x = if physical.previous_position.x <= min.x {
            min.x + min_distance
        } else {
            physical.previous_position.x
        };

        let mut velocity = *rigidbody.linvel();
        velocity.x = 0.0;
        rigidbody.set_linvel(velocity, true);
    } else if transform.translation.x >= max.x {
        transform.translation.x = if physical.previous_position.x >= max.x {
            max.x - min_distance
        } else {
            physical.previous_position.x
        };

        let mut velocity = *rigidbody.linvel();
        velocity.x = 0.0;
        rigidbody.set_linvel(velocity, true);
    }

    if transform.translation.y <= min.y {
        transform.translation.y = if physical.previous_position.y <= min.y {
            min.y + min_distance
        } else {
            physical.previous_position.y
        };

        let mut velocity = *rigidbody.linvel();
        velocity.y = 0.0;
        rigidbody.set_linvel(velocity, true);
    } else if transform.translation.y >= max.y {
        transform.translation.y = if physical.previous_position.y >= max.y {
            max.y - min_distance
        } else {
            physical.previous_position.y
        };

        let mut velocity = *rigidbody.linvel();
        velocity.y = 0.0;
        rigidbody.set_linvel(velocity, true);
    }
}

fn attract_repel_force(
    rigidbody: &RigidBody,
    ab: Vec2,
    acceleration: f32,
    min_distance: f32,
) -> Vec2 {
    let distance = Float::max(min_distance, ab.length());
    let direction = ab.normalize_or_zero();
    let magnitude = (acceleration * rigidbody.mass()) / (distance * distance);

    direction * magnitude
}

/// Repel a rigidbody inside bounds
#[allow(dead_code)]
pub fn bounds_repel(
    rigidbody: &mut RigidBody,
    transform: &Transform,
    min: Vec2,
    max: Vec2,
    acceleration: f32,
    min_distance: f32,
) {
    let force = attract_repel_force(
        rigidbody,
        Vec2::new(transform.translation.x - min.x, 0.0),
        acceleration,
        min_distance,
    );
    rigidbody.apply_force(to_vector(force), true);

    let force = attract_repel_force(
        rigidbody,
        Vec2::new(transform.translation.x - max.x, 0.0),
        acceleration,
        min_distance,
    );
    rigidbody.apply_force(to_vector(force), true);

    let force = attract_repel_force(
        rigidbody,
        Vec2::new(0.0, transform.translation.y - min.y),
        acceleration,
        min_distance,
    );
    rigidbody.apply_force(to_vector(force), true);

    let force = attract_repel_force(
        rigidbody,
        Vec2::new(0.0, transform.translation.y - max.y),
        acceleration,
        min_distance,
    );
    rigidbody.apply_force(to_vector(force), true);
}

/// Repel a a rigidbody away from a point
#[allow(dead_code)]
pub fn repel(
    rigidbody: &mut RigidBody,
    transform: &Transform,
    point: Vec2,
    acceleration: f32,
    min_distance: f32,
) {
    let force = attract_repel_force(
        rigidbody,
        transform.translation.truncate() - point,
        acceleration,
        min_distance,
    );
    rigidbody.apply_force(to_vector(force), true);
}

/// Adjusts the bounds that should contain a collider
pub fn adjust_container_bounds(size: Vec2, min: Vec2, max: Vec2, offset: f32) -> (Vec2, Vec2) {
    let size = size;
    let offset = Vec2::splat(offset);
    let min = min + size + offset;
    let max = max - size - offset;

    (min, max)
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
    pub fn update(&mut self, dt: f32, transform: &mut Transform) {
        self.angle += self.velocity * dt;

        transform.translation.x = self.angle.x.sin() * self.amplitude.x;
        transform.translation.y = self.angle.y.sin() * self.amplitude.y;
    }
}
