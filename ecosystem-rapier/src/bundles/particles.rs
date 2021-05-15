//! Particle system bundles

use bevy::prelude::*;

use crate::components::particles::*;
use crate::resources::*;

/// Particle bundle
#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,

    transform: Transform,
    global_transform: GlobalTransform,
}

impl ParticleBundle {
    pub fn new(
        random: &mut Random,
        transform: Transform,
        particle_system: &ParticleSystem,
    ) -> Self {
        Self {
            particle: Particle::new(random, particle_system),
            transform,
            global_transform: GlobalTransform::default(),
        }
    }
}
