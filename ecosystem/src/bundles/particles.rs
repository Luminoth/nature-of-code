//! Particle system bundles

use bevy::prelude::*;

use crate::components::particles::*;
use crate::resources::*;

/// Particle bundle
#[derive(Bundle)]
pub struct ParticleBundle {
    transform: Transform,
    global_transform: GlobalTransform,

    particle: Particle,
}

impl ParticleBundle {
    pub fn new(random: &mut Random, transform: Transform, lifespan: f32, max_speed: f32) -> Self {
        Self {
            transform,
            global_transform: GlobalTransform::default(),
            particle: Particle::new(random, lifespan, max_speed),
        }
    }
}
