//! ECS components

pub mod creatures;
pub mod debug;

use bevy::prelude::*;

/// Physics state
pub struct Physics {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,

    pub max_speed: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            acceleration: Vec3::default(),
            velocity: Vec3::default(),
            mass: 1.0,
            max_speed: 1.0,
        }
    }
}

impl Physics {
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
        // clamped euler integration
        self.velocity = (self.velocity + self.acceleration).clamp_length_max(self.max_speed);
        transform.translation += self.velocity;
    }
}
