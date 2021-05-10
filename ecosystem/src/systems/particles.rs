//! Particle system systems

use bevy::prelude::*;

use crate::components::particles::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ParticlesSystem {
    ParticleSystems,
    Particles,
}

pub fn update_particle_systems(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut ParticleSystem>,
) {
    for mut particle_system in query.iter_mut() {
        particle_system.update(&mut commands, &time);
    }
}

pub fn update_particles(mut query: Query<&mut Particle>) {
    for mut _particle in query.iter_mut() {
        //particle.update();
    }
}
