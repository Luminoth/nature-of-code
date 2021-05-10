//! Particle system bundles

use bevy::prelude::*;

use crate::components::particles::*;

/// Particle bundle
#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
}

impl ParticleBundle {
    pub fn new(lifespan: f32) -> Self {
        Self {
            particle: Particle::new(lifespan),
        }
    }
}
