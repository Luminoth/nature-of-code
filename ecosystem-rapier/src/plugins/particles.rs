//! Particle system plugin

use bevy::core::FixedTimestep;
use bevy::ecs::component::{ComponentDescriptor, StorageType};
use bevy::prelude::*;
use bevy_inspector_egui::InspectableRegistry;

use crate::components::particles::*;
use crate::components::physics::*;
use crate::systems::particles::*;

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // sparse storage since we add / remove components on these objects
        app.register_component(ComponentDescriptor::new::<Particle>(StorageType::SparseSet))
            // per-frame update
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
            // fixed (physics) update
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    .with_system(update_particle_physics.system()),
            );

        // register components for inspector
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);

        registry.register::<ParticleSystem>();
        registry.register::<Particle>();
    }
}
