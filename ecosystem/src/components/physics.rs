//! Physics components

use bevy::prelude::*;

/// Rigidbody state
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
    pub fn wrap(&mut self, transform: &mut Transform, minx: f32, maxx: f32, miny: f32, maxy: f32) {
        if !transform.translation.is_finite() {
            panic!("Invalid transform {}", transform.translation);
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
    pub fn contain(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
        if !transform.translation.is_finite() {
            panic!("Invalid transform {}", transform.translation);
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
    pub fn bounce(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
        if !transform.translation.is_finite() {
            panic!("Invalid transform {}", transform.translation);
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

    /// Applies a force to the rigidbody
    pub fn apply_force(&mut self, force: Vec2, _name: impl AsRef<str>) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        //info!("applying force '{}': {}", _name.as_ref(), force);

        self.acceleration += force.extend(0.0);
        if !self.acceleration.is_finite() {
            panic!("Invalid acceleration from force {}", force);
        }

        //info!("updated acceleration: {}", self.acceleration);
    }

    /// Update a rigidbody
    pub fn update(&mut self, transform: &mut Transform, dt: f32) {
        self.velocity += self.acceleration * dt;
        if !self.velocity.is_finite() {
            panic!(
                "Invalid velocity from acceleration {} at slice {}",
                self.acceleration, dt
            );
        }

        //info!("updated velocity: {}", self.velocity);

        transform.translation += self.velocity * dt;
        if !transform.translation.is_finite() {
            panic!(
                "Invalid position from velocity {} and at {}",
                self.velocity, dt
            );
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
#[derive(Default)]
pub struct Surface {
    pub c: f32,
}

impl Surface {
    /// Constructs a new surface with the given coefficient of friction
    pub fn new(c: f32) -> Self {
        Self { c }
    }
}

/// Fluid state
#[derive(Default)]
pub struct Fluid {
    pub density: f32,
}

impl Fluid {
    /// Constructs a new fluid with the given drag coefficient
    pub fn new(density: f32) -> Self {
        Self { density }
    }
}
