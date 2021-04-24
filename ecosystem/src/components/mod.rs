//! ECS components

pub mod creatures;

use bevy::prelude::*;

/// Physics state
#[derive(Default)]
pub struct Physics {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub max_speed: f32,
}

impl Physics {
    /// Wrap a physical around bounds
    #[allow(dead_code)]
    pub fn wrap(transform: &mut Transform, minx: f32, maxx: f32, miny: f32, maxy: f32) {
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
    pub fn contain(transform: &mut Transform, minx: f32, maxx: f32, miny: f32, maxy: f32) {
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

    /// Update a physical
    pub fn update(&mut self, transform: &mut Transform) {
        // clamped euler integration
        self.velocity = (self.velocity + self.acceleration).clamp_length_max(self.max_speed);
        transform.translation += self.velocity;
    }
}
