//! Particle system bundles

use bevy::prelude::*;

use crate::components::particles::*;

/// Particle bundle
#[derive(Bundle)]
pub struct ParticleBundle {
    transform: Transform,
    global_transform: GlobalTransform,

    particle: Particle,
}

impl ParticleBundle {
    pub fn new(transform: Transform, lifespan: f32) -> Self {
        Self {
            transform,
            global_transform: GlobalTransform::default(),
            particle: Particle::new(lifespan),
        }
    }
}
