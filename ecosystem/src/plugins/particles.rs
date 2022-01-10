//! Particle system plugins

use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::components::physics::*;
use crate::systems::particles::*;

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(update_particle_systems.label(ParticlesSystem::ParticleSystems))
                .with_system(
                    update_particles
                        .label(ParticlesSystem::Particles)
                        .before(ParticlesSystem::ParticleSystems),
                ),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                .with_system(update_particle_physics),
        );
    }
}
