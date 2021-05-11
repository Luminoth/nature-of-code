//! Particle system components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::particles::*;

use super::physics::*;

/// Particle system component
#[derive(Debug, Inspectable, Default)]
pub struct ParticleSystem {
    #[inspectable(read_only)]
    name: String,

    #[inspectable(read_only)]
    capacity: usize,

    pub spawn_rate: f64,
    pub particle_lifespan: f32,

    #[inspectable(read_only)]
    next_spawn: f64,

    #[inspectable(ignore)]
    dead: Vec<Entity>,

    #[inspectable(ignore)]
    live: Vec<Entity>,
}

impl ParticleSystem {
    /// Create a new particle system with a pool of the given capacity
    pub fn with_capacity(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            capacity,
            spawn_rate: 1.0,
            particle_lifespan: 1.0,
            next_spawn: 0.0,
            dead: Vec::with_capacity(capacity),
            live: Vec::with_capacity(capacity),
        }
    }

    fn spawn(&mut self, commands: &mut Commands) {
        for _ in 0..self.capacity {
            let entity = commands.spawn().insert(Name::new("Particle")).id();
            self.dead.push(entity);
        }
    }

    /// Spawn a new particle
    ///
    /// Grows the pool if necessary
    pub fn spawn_particle(&mut self, commands: &mut Commands) {
        // grow if we need to, this is pretty expensive
        if self.dead.is_empty() {
            debug!("Growing particle system {}", self.name);
            self.dead.reserve(self.capacity);
            self.spawn(commands);
        }

        let entity = self.dead.pop().unwrap();
        commands
            .entity(entity)
            .insert_bundle(ParticleBundle::new(self.particle_lifespan));
    }

    /// Updates the particle system
    pub fn update(&mut self, commands: &mut Commands, time: &Time) {
        let now = time.seconds_since_startup();
        if now >= self.next_spawn {
            self.spawn_particle(commands);

            self.next_spawn = now + self.spawn_rate;
        }

        // drain_filter() equivalent
        /*let mut i = 0;
        while i != self.live.len() {
            let particle = &mut self.live[i];
            if particle.is_dead() {
                // TODO: remove the particle bundle from the entity
                self.dead.push(*particle);
                self.live.remove(i);
            } else {
                i += 1;
            }
        }*/
    }
}

/// Particle component
#[derive(Debug, Inspectable)]
pub struct Particle {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub lifespan: f32,
}

impl Particle {
    /// Creates a new particle with the given lifespan
    pub fn new(lifespan: f32) -> Self {
        Self {
            acceleration: Vec3::default(),
            velocity: Vec3::default(),
            lifespan,
        }
    }

    /// Is this particle dead?
    pub fn is_dead(&self) -> bool {
        self.lifespan <= 0.0
    }

    /// Updates the particle
    pub fn update(&mut self, dt: f32) {
        self.lifespan -= dt;

        // TODO: particles need a reference to the system that owns them
        // if they die, they need to tell the system to remove the particle
    }

    /// Updates the particle physics
    pub fn update_physics(&mut self, transform: &mut Transform) {
        // https://github.com/bevyengine/bevy/issues/2041
        let dt = PHYSICS_STEP;

        //sympletic_euler_integrate(transform, self.acceleration, &mut self.velocity, dt);
        rk4_integrate(transform, self.acceleration, &mut self.velocity, dt);

        self.acceleration = Vec3::default();
    }
}
