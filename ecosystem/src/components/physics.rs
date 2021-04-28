//! Physics components

use bevy::prelude::*;
use bevy::utils::tracing;

/// Physics step rate
/// 50hz, the same as Unity
pub const PHYSICS_STEP: f32 = 0.02;

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

    /// Gets the rigidbody speed squaed
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

    /// Update a rigidbody
    #[tracing::instrument]
    pub fn update(&mut self, transform: &mut Transform, mut dt: f32) {
        // this can happen over the first couple seconds of runtime, not really sure why
        // for those frames tho, just treat it as a single step
        if dt < PHYSICS_STEP - 0.05 || dt > PHYSICS_STEP + 0.05 {
            info!("unexpected physics step, expected {}", PHYSICS_STEP);
            dt = PHYSICS_STEP;
        }

        self.velocity += self.acceleration * dt;
        if !self.velocity.is_finite() {
            panic!("Invalid velocity from acceleration");
        }

        //info!("updated velocity: {}", self.velocity);

        transform.translation += self.velocity * dt;
        if !transform.translation.is_finite() {
            panic!("Invalid position from velocity");
        }

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
