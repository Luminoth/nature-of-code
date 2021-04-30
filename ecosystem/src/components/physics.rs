//! Physics components

use bevy::prelude::*;
use bevy::utils::tracing;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

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
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub drag: f32,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
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
        let speed = self.velocity.length();
        // TODO: this shouldn't be possible and yet it happens?
        if !speed.is_finite() {
            0.0
        } else {
            speed
        }
    }

    /// Gets the rigidbody speed squared
    #[allow(dead_code)]
    pub fn speed_squared(&self) -> f32 {
        let speed_squared = self.velocity.length_squared();
        // TODO: this shouldn't be possible and yet it happens?
        if !speed_squared.is_finite() {
            0.0
        } else {
            speed_squared
        }
    }

    /// Wrap a rigidbody around bounds
    #[allow(dead_code)]
    #[tracing::instrument]
    pub fn wrap(&mut self, transform: &mut Transform, minx: f32, maxx: f32, miny: f32, maxy: f32) {
        if !transform.translation.is_finite() {
            panic!("Invalid transform");
        }

        if transform.translation.x < minx {
            transform.translation.x = maxx;
        } else if transform.translation.x > maxx {
            transform.translation.x = minx;
        }

        if transform.translation.y < miny {
            transform.translation.y = maxy;
        } else if transform.translation.y > maxy {
            transform.translation.y = miny;
        }
    }

    /// Contain a rigidbody inside bounds
    #[allow(dead_code)]
    #[tracing::instrument]
    pub fn contain(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
        if !transform.translation.is_finite() {
            panic!("Invalid transform");
        }

        if transform.translation.x < minx {
            transform.translation.x = minx;
        } else if transform.translation.x > maxx {
            transform.translation.x = maxx;
        }

        if transform.translation.y < miny {
            transform.translation.y = miny;
        } else if transform.translation.y > maxy {
            transform.translation.y = maxy;
        }
    }

    /// Bounce a rigidbody inside bounds
    #[allow(dead_code)]
    #[tracing::instrument]
    pub fn bounce(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
        if !transform.translation.is_finite() {
            panic!("Invalid transform");
        }

        if transform.translation.x < minx {
            transform.translation.x = minx;

            self.acceleration.x *= -1.0;
            self.velocity.x *= -1.0;
        } else if transform.translation.x > maxx {
            transform.translation.x = maxx;

            self.acceleration.x *= -1.0;
            self.velocity.x *= -1.0;
        }

        if transform.translation.y < miny {
            transform.translation.y = miny;

            self.acceleration.y *= -1.0;
            self.velocity.y *= -1.0;
        } else if transform.translation.y > maxy {
            transform.translation.y = maxy;

            self.acceleration.y *= -1.0;
            self.velocity.y *= -1.0;
        }
    }

    /// Repel a rigidbody inside bounds
    #[allow(dead_code)]
    pub fn repel(
        &mut self,
        _transform: &mut Transform,
        _minx: f32,
        _maxx: f32,
        _miny: f32,
        _maxy: f32,
    ) {
        // TODO:
    }

    /// Applies a force to the rigidbody
    #[tracing::instrument]
    pub fn apply_force(&mut self, force: Vec2, _name: impl AsRef<str> + std::fmt::Debug) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        //info!("applying force '{}': {}", _name.as_ref(), force);

        self.acceleration += force.extend(0.0);
        if !self.acceleration.is_finite() {
            panic!("Invalid acceleration from force");
        }

        //info!("updated acceleration: {}", self.acceleration);
    }

    /// RK4 integration: https://gafferongames.com/post/integration_basics/
    #[allow(dead_code)]
    #[tracing::instrument]
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
        if !self.velocity.is_finite() {
            panic!("Invalid velocity from acceleration");
        }

        //info!("updated velocity: {}", self.velocity);

        transform.translation += dxdt * dt;
        if !transform.translation.is_finite() {
            panic!("Invalid position from velocity");
        }
    }

    /// Explicit Euler integration
    #[allow(dead_code)]
    #[tracing::instrument]
    fn explicit_euler_integrate(&mut self, transform: &mut Transform, dt: f32) {
        transform.translation += self.velocity * dt;
        if !transform.translation.is_finite() {
            panic!("Invalid position from velocity");
        }

        self.velocity += self.acceleration * dt;
        if !self.velocity.is_finite() {
            panic!("Invalid velocity from acceleration");
        }

        //info!("updated velocity: {}", self.velocity);
    }

    /// Semi-implicit Euler integration
    #[allow(dead_code)]
    #[tracing::instrument]
    fn sympletic_euler_integrate(&mut self, transform: &mut Transform, dt: f32) {
        self.velocity += self.acceleration * dt;
        if !self.velocity.is_finite() {
            panic!("Invalid velocity from acceleration");
        }

        //info!("updated velocity: {}", self.velocity);

        transform.translation += self.velocity * dt;
        if !transform.translation.is_finite() {
            panic!("Invalid position from velocity");
        }
    }

    /// Update a rigidbody
    #[tracing::instrument]
    pub fn update(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

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
    /// Constructs a new collider
    pub fn new(layer: CollisionLayer, width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
            layer,
        }
    }

    /// Check if a collider is colliding with another collider
    #[tracing::instrument]
    pub fn collides(
        &self,
        _transform: &Transform,
        other: &Collider,
        _other_transform: &Transform,
    ) -> bool {
        // TODO: also check overlapping bounds
        self.layer == other.layer
    }
}

/// Surface state
#[derive(Debug, Default)]
pub struct Surface {
    pub c: f32,
}

impl Surface {
    /// Constructs a new surface with the given coefficient of friction
    pub fn new(c: f32) -> Self {
        Self { c }
    }

    #[tracing::instrument]
    pub fn update(&self, rigidbody: &mut Rigidbody) {
        let magnitude = self.c;
        let direction = -rigidbody.velocity.normalize_or_zero();

        let friction = direction * magnitude;
        if !friction.is_finite() {
            panic!("Invalid friction");
        }

        rigidbody.apply_force(friction.truncate(), "friction");
    }
}

/// Fluid state
#[derive(Debug, Default)]
pub struct Fluid {
    pub density: f32,
}

impl Fluid {
    /// Constructs a new fluid with the given drag coefficient
    pub fn new(density: f32) -> Self {
        Self { density }
    }

    #[tracing::instrument]
    pub fn update(&self, rigidbody: &mut Rigidbody) {
        let speed_squared = rigidbody.speed_squared();
        let magnitude = 0.5 * self.density * speed_squared * rigidbody.drag;
        let direction = -rigidbody.velocity.normalize_or_zero();

        let drag = direction * magnitude;
        if !drag.is_finite() {
            panic!("Invalid drag");
        }

        //info!("drag: {} for speed {}", drag, speed_squared);

        rigidbody.apply_force(drag.truncate(), "drag");
    }
}
