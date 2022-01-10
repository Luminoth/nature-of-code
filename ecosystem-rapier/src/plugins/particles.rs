//! Particle system plugin

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use crate::components::particles::*;
use crate::components::physics::*;
use crate::systems::particles::*;

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut App) {
        // sparse storage since we add / remove components on these objects
        app
            // per-frame update
            .add_system_set(
                SystemSet::new()
                    .with_system(update_particle_systems.label(ParticlesSystem::ParticleSystems))
                    .with_system(
                        update_particles
                            .label(ParticlesSystem::Particles)
                            .before(ParticlesSystem::ParticleSystems),
                    ),
            )
            // fixed (physics) update
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                    .with_system(update_particle_physics),
            );

        // register components for inspector
        app.register_inspectable::<ParticleSystem>()
            .register_inspectable::<Particle>();
    }
}
