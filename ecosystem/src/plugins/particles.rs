//! Particle system plugins

use bevy::ecs::component::{ComponentDescriptor, StorageType};
use bevy::prelude::*;

use crate::components::particles::*;
use crate::systems::particles::*;

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_component(ComponentDescriptor::new::<Particle>(StorageType::SparseSet))
            // TODO: this isn't working for particle systems spawned *after* this runs
            .add_startup_system(setup_particle_systems.system())
            .add_system(
                update_particle_systems
                    .system()
                    .label(ParticlesSystem::ParticleSystems),
            )
            .add_system(
                update_particles
                    .system()
                    .label(ParticlesSystem::Particles)
                    .before(ParticlesSystem::ParticleSystems),
            );
    }
}
