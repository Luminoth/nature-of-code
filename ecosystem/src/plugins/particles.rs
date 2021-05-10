//! Particle system plugins

use bevy::core::FixedTimestep;
use bevy::ecs::component::{ComponentDescriptor, StorageType};
use bevy::prelude::*;

use crate::components::particles::*;
use crate::components::physics::*;
use crate::systems::particles::*;

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_component(ComponentDescriptor::new::<Particle>(StorageType::SparseSet))
            .add_system_set(
                SystemSet::new()
                    .with_system(
                        update_particle_systems
                            .system()
                            .label(ParticlesSystem::ParticleSystems),
                    )
                    .with_system(
                        update_particles
                            .system()
                            .label(ParticlesSystem::Particles)
                            .before(ParticlesSystem::ParticleSystems),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    .with_system(update_particle_physics.system()),
            );
    }
}
