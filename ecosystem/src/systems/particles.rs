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
    mut query: Query<(&Transform, &mut ParticleSystem)>,
    particles: Query<&Particle>,
) {
    for (transform, mut particle_system) in query.iter_mut() {
        particle_system.update(&mut commands, &time, &transform, &particles);
    }
}

/// Updates all of the particles
pub fn update_particles(
    time: Res<Time>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut query: Query<(&mut Particle, &Handle<Mesh>)>,
    mut query: Query<&mut Particle>,
) {
    //for (mut particle, mesh) in query.iter_mut() {
    for mut particle in query.iter_mut() {
        particle.update(time.delta_seconds() /*, &mut meshes, &mesh*/);
    }
}

/// Updates particle physics
pub fn update_particle_physics(mut query: Query<(&mut Transform, &mut Particle)>) {
    for (mut transform, mut particle) in query.iter_mut() {
        particle.update_physics(&mut transform);
    }
}
