//! Particle system components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::particles::*;

/// Particle system component
#[derive(Debug, Inspectable, Default)]
pub struct ParticleSystem {
    #[inspectable(read_only)]
    capacity: usize,

    pub lifespan: f32,

    #[inspectable(ignore)]
    dead: Vec<Entity>,

    #[inspectable(ignore)]
    live: Vec<Entity>,
}

impl ParticleSystem {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            lifespan: 1.0,
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

    pub fn setup(&mut self, commands: &mut Commands) {
        self.spawn(commands);
    }

    pub fn spawn_particle(&mut self, commands: &mut Commands) {
        // grow if we need to, this is pretty expensive
        if self.dead.is_empty() {
            self.dead.reserve(self.capacity);
            self.spawn(commands);
        }

        let entity = self.dead.pop().unwrap();
        commands
            .entity(entity)
            .insert_bundle(ParticleBundle::new(self.lifespan));
    }

    pub fn update(&mut self, commands: &mut Commands) {
        //self.spawn_particle(commands);

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
    pub lifespan: f32,
}

impl Particle {
    pub fn new(lifespan: f32) -> Self {
        Self { lifespan }
    }

    pub fn is_dead(&self) -> bool {
        self.lifespan <= 0.0
    }

    pub fn update(&mut self, dt: f32) {
        self.lifespan -= dt;
    }
}
