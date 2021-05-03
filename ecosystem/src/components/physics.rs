//! Physics components

use bevy::prelude::*;
use num_traits::Float;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

// TODO: for creates this should be their radius
const ATTRACT_REPEL_MIN_DISTANCE: f32 = 0.1;

#[derive(Debug, Default, Copy, Clone)]
struct Derivative {
    acceleration: Vec3,
    velocity: Vec3,
}

impl Derivative {
    fn evaluate(rigidbody: &Rigidbody, dt: f32, derivative: Self) -> Self {
        Self {
            velocity: rigidbody.velocity + derivative.acceleration * dt,
            acceleration: rigidbody.acceleration,
        }
    }
}

/// Rigidbody state
#[derive(Debug)]
pub struct Rigidbody {
    pub(crate) previous_position: Vec3,

    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub drag: f32,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            previous_position: Vec3::default(),

            acceleration: Vec3::default(),
            velocity: Vec3::default(),
            mass: 1.0,
            drag: 0.0,
        }
    }
}

impl Rigidbody {
    /// Gets the rigidbody speed
    #[allow(dead_code)]
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    /// Gets the rigidbody speed squared
    #[allow(dead_code)]
    pub fn speed_squared(&self) -> f32 {
        self.velocity.length_squared()
    }

    /// Contain a rigidbody inside bounds
    #[allow(dead_code)]
    pub fn contain(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
        // unwind to our previous position, if we can
        // otherwise clamp to the min / max minus a little fudge

        if transform.translation.x <= minx {
            transform.translation.x = if self.previous_position.x <= minx {
                minx + ATTRACT_REPEL_MIN_DISTANCE
            } else {
                self.previous_position.x
            };
            self.velocity.x = 0.0;
        } else if transform.translation.x >= maxx {
            transform.translation.x = if self.previous_position.x >= maxx {
                maxx - ATTRACT_REPEL_MIN_DISTANCE
            } else {
                self.previous_position.x
            };
            self.velocity.x = 0.0;
        }

        if transform.translation.y <= miny {
            transform.translation.y = if self.previous_position.y <= miny {
                miny + ATTRACT_REPEL_MIN_DISTANCE
            } else {
                self.previous_position.y
            };
            self.velocity.y = 0.0;
        } else if transform.translation.y >= maxy {
            transform.translation.y = if self.previous_position.y >= maxy {
                maxy - ATTRACT_REPEL_MIN_DISTANCE
            } else {
                self.previous_position.y
            };
            self.velocity.y = 0.0;
        }
    }

    fn attract_repel_force(&self, ab: Vec2, acceleration: f32) -> Vec2 {
        let distance = Float::max(ATTRACT_REPEL_MIN_DISTANCE, ab.length());
        let direction = ab.normalize_or_zero();
        let magnitude = (acceleration * self.mass) / (distance * distance);

        direction * magnitude
    }

    /// Repel a rigidbody inside bounds
    #[allow(dead_code)]
    pub fn bounds_repel(
        &mut self,
        transform: &Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
        acceleration: f32,
    ) {
        let force =
            self.attract_repel_force(Vec2::new(transform.translation.x - minx, 0.0), acceleration);
        self.apply_force(force);

        let force =
            self.attract_repel_force(Vec2::new(transform.translation.x - maxx, 0.0), acceleration);
        self.apply_force(force);

        let force =
            self.attract_repel_force(Vec2::new(0.0, transform.translation.y - miny), acceleration);
        self.apply_force(force);

        let force =
            self.attract_repel_force(Vec2::new(0.0, transform.translation.y - maxy), acceleration);
        self.apply_force(force);
    }

    /// Repel a a rigidbody away from a point
    #[allow(dead_code)]
    pub fn repel(&mut self, transform: &Transform, point: Vec2, acceleration: f32) {
        let force =
            self.attract_repel_force(transform.translation.truncate() - point, acceleration);
        self.apply_force(force);
    }

    /// Attact a a rigidbody toward a point
    #[allow(dead_code)]
    pub fn atract(&mut self, transform: &Transform, point: Vec2, acceleration: f32) {
        let force =
            self.attract_repel_force(point - transform.translation.truncate(), acceleration);
        self.apply_force(force);
    }

    /// Applies a force to the rigidbody
    pub fn apply_force(&mut self, force: Vec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        self.acceleration += force.extend(0.0);
    }

    /// RK4 integration: https://gafferongames.com/post/integration_basics/
    #[allow(dead_code)]
    fn rk4_integrate(&mut self, transform: &mut Transform, dt: f32) {
        // sample derivative at four points
        let a = Derivative::evaluate(self, 0.0, Derivative::default());
        let b = Derivative::evaluate(self, dt * 0.5, a);
        let c = Derivative::evaluate(self, dt * 0.5, b);
        let d = Derivative::evaluate(self, dt, c);

        // taylor expansion weighted sum
        let dvdt =
            1.0 / 6.0 * (a.acceleration + 2.0 * (b.acceleration + c.acceleration) + d.acceleration);
        let dxdt = 1.0 / 6.0 * (a.velocity + 2.0 * (b.velocity + c.velocity) + d.velocity);

        self.velocity += dvdt * dt;
        transform.translation += dxdt * dt;
    }

    /// Explicit Euler integration
    #[allow(dead_code)]
    fn explicit_euler_integrate(&mut self, transform: &mut Transform, dt: f32) {
        transform.translation += self.velocity * dt;
        self.velocity += self.acceleration * dt;
    }

    /// Semi-implicit Euler integration
    #[allow(dead_code)]
    fn sympletic_euler_integrate(&mut self, transform: &mut Transform, dt: f32) {
        self.velocity += self.acceleration * dt;
        transform.translation += self.velocity * dt;
    }

    /// Update a rigidbody
    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        // save our current position in case we need to unwind
        self.previous_position = transform.translation;

        //self.sympletic_euler_integrate(transform, dt);
        self.rk4_integrate(transform, dt);

        self.acceleration = Vec3::default();
    }
}

/// Colliders on the same layer collide
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CollisionLayer {
    Water,
    Ground,
    Air,
}

/// Colliders collide
#[derive(Debug)]
pub struct Collider {
    pub size: Vec2,
    pub layer: CollisionLayer,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            size: Vec2::new(1.0, 1.0),
            layer: CollisionLayer::Ground,
        }
    }
}

impl Collider {
    /// Check if a collider is colliding with another collider
    pub fn collides(
        &self,
        _transform: &Transform,
        other: &Collider,
        _other_transform: &Transform,
    ) -> bool {
        // TODO: also check overlapping bounds
        self.layer == other.layer
    }

    /// Adjusts the bounds that should contain this collider
    pub fn adjust_container_bounds(
        &self,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
        offset: f32,
    ) -> (f32, f32, f32, f32) {
        let minx = minx + self.size.x + offset;
        let maxx = maxx - self.size.x - offset;
        let miny = miny + self.size.y + offset;
        let maxy = maxy - self.size.y - offset;

        (minx, maxx, miny, maxy)
    }
}

/// Surface state
#[derive(Debug, Default)]
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
#[derive(Debug, Default)]
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
