//! Particle system systems

use bevy::prelude::*;

use crate::components::particles::*;
use crate::resources::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ParticlesSystem {
    ParticleSystems,
    Particles,
}

/// Updates the particle systems
pub fn update_particle_systems(
    mut commands: Commands,
    time: Res<Time>,
    mut random: ResMut<Random>,
    mut query: Query<(&GlobalTransform, &mut ParticleSystem)>,
    particles: Query<&Particle>,
) {
    for (global_transform, mut particle_system) in query.iter_mut() {
        let transform = (*global_transform).into();
        particle_system.update(&mut commands, &time, &mut random, &transform, &particles);
    }
}

/// Updates all of the particles
pub fn update_particles(time: Res<Time>, mut query: Query<&mut Particle>) {
    let dt = time.delta_seconds();

    query.for_each_mut(|mut particle| {
        particle.update(dt);
    });
}

/// Updates particle physics
pub fn update_particle_physics(mut query: Query<(&mut Transform, &mut Particle)>) {
    query.for_each_mut(|(mut transform, mut particle)| {
        particle.update_physics(&mut transform);
    });
}
