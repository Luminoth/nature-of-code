//! Particle system systems

use bevy::prelude::*;

use crate::components::particles::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ParticlesSystem {
    ParticleSystems,
    Particles,
}

/// Updates the particle systems
pub fn update_particle_systems(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut ParticleSystem>,
) {
    for mut particle_system in query.iter_mut() {
        particle_system.update(&mut commands, &time);
    }
}

/// Updates all of the particles
pub fn update_particles(time: Res<Time>, mut query: Query<&mut Particle>) {
    for mut particle in query.iter_mut() {
        particle.update(time.delta_seconds());
    }
}

/// Updates particle physics
pub fn update_particle_physics(mut query: Query<(&mut Transform, &mut Particle)>) {
    for (mut transform, mut particle) in query.iter_mut() {
        particle.update_physics(&mut transform);
    }
}
