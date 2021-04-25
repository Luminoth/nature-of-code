//! Physics components

use bevy::prelude::*;

/// Rigidbody state
pub struct Rigidbody {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            acceleration: Vec3::default(),
            velocity: Vec3::default(),
            mass: 1.0,
        }
    }
}

impl Rigidbody {
    /// Wrap a physical around bounds
    #[allow(dead_code)]
    pub fn wrap(&mut self, transform: &mut Transform, minx: f32, maxx: f32, miny: f32, maxy: f32) {
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

    /// Contain a physical inside bounds
    #[allow(dead_code)]
    pub fn contain(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
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

    /// Contain a physical inside bounds
    #[allow(dead_code)]
    pub fn bounce(
        &mut self,
        transform: &mut Transform,
        minx: f32,
        maxx: f32,
        miny: f32,
        maxy: f32,
    ) {
        if transform.translation.x < minx {
            transform.translation.x = minx;
            self.velocity.x *= -1.0;
        } else if transform.translation.x > maxx {
            transform.translation.x = maxx;
            self.velocity.x *= -1.0;
        }

        if transform.translation.y < miny {
            transform.translation.y = miny;
            self.velocity.y *= -1.0;
        } else if transform.translation.y > maxy {
            transform.translation.y = maxy;
            self.velocity.y *= -1.0;
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        let force = force / self.mass;
        self.acceleration += Vec3::from((force, 0.0));
    }

    /// Update a physical
    pub fn update(&mut self, transform: &mut Transform) {
        // euler integration
        self.velocity += self.acceleration;
        transform.translation += self.velocity;

        self.acceleration = Vec3::default();
    }
}

pub struct Collider {
    // TODO: AABB
}

impl Default for Collider {
    fn default() -> Self {
        Self {}
    }
}

/// Surface state
#[derive(Default)]
pub struct Surface {
    pub c: f32,
}

/// Fluid state
#[derive(Default)]
pub struct Fluid {
    pub c: f32,
}
