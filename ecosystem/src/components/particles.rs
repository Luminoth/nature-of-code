//! Particle system components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::bundles::particles::*;
use crate::resources::*;

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
    pub max_speed: f32,
    pub material: Handle<ColorMaterial>,

    #[inspectable(read_only)]
    next_spawn: f64,

    #[inspectable(ignore)]
    pool: Vec<Entity>,

    #[inspectable(ignore)]
    live: Vec<Entity>,
}

impl ParticleSystem {
    /// Create a new particle system with a pool of the given capacity
    pub fn with_capacity(
        name: impl Into<String>,
        material: Handle<ColorMaterial>,
        capacity: usize,
    ) -> Self {
        Self {
            name: name.into(),
            capacity,
            spawn_rate: 1.0,
            particle_lifespan: 1.0,
            material,
            max_speed: 1.0,
            next_spawn: 0.0,
            pool: Vec::with_capacity(capacity),
            live: Vec::with_capacity(capacity),
        }
    }

    #[allow(dead_code)]
    pub fn apply_force(&mut self, particles: &mut Query<&mut Particle>, force: Vec2) {
        for entity in &self.live {
            let mut particle = particles.get_mut(*entity).unwrap();
            particle.apply_force(force);
        }
    }

    fn spawn(&mut self, commands: &mut Commands) {
        for _ in 0..self.capacity {
            let entity = commands.spawn().insert(Name::new("Particle")).id();
            self.pool.push(entity);
        }
    }

    /// Spawn a new particle
    ///
    /// Grows the pool if necessary
    fn spawn_particle(
        &mut self,
        commands: &mut Commands,
        random: &mut Random,
        transform: Transform,
    ) {
        // grow if we need to, this is pretty expensive
        if self.pool.is_empty() {
            debug!("Growing particle system {}", self.name);
            self.pool.reserve(self.capacity);
            self.spawn(commands);
        }

        let entity = self.pool.pop().unwrap();
        commands
            .entity(entity)
            .insert_bundle(ParticleBundle::new(
                random,
                transform,
                self.particle_lifespan,
                self.max_speed,
            ))
            // TODO: this should be a child of the particle
            // but not sure how to remove it if we do that
            .insert_bundle(SpriteBundle {
                material: self.material.clone(),
                sprite: Sprite::new(Vec2::splat(0.05)),
                transform,
                ..Default::default()
            });
        self.live.push(entity);
    }

    /// Updates the particle system
    pub fn update(
        &mut self,
        commands: &mut Commands,
        time: &Time,
        random: &mut Random,
        transform: &Transform,
        particles: &Query<&Particle>,
    ) {
        // remove dead particles first
        // drain_filter() equivalent
        let mut i = 0;
        while i != self.live.len() {
            let entity = self.live[i];
            let particle = particles.get(entity).unwrap();
            if particle.is_dead() {
                commands
                    .entity(entity)
                    .remove_bundle::<ParticleBundle>()
                    .remove_bundle::<SpriteBundle>();

                self.pool.push(entity);
                self.live.remove(i);
            } else {
                i += 1;
            }
        }

        // spawn new particles last
        let now = time.seconds_since_startup();
        if now >= self.next_spawn {
            self.spawn_particle(commands, random, *transform);

            self.next_spawn = now + self.spawn_rate;
        }
    }
}

/// Particle component
#[derive(Debug, Inspectable)]
pub struct Particle {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub drag: f32,

    pub lifespan: f32,
    pub health: f32,
}

impl Particle {
    /// Creates a new particle with the given lifespan
    pub fn new(random: &mut Random, lifespan: f32, max_speed: f32) -> Self {
        Self {
            acceleration: Vec3::default(),
            velocity: Vec3::new(
                random.random_range(-max_speed..=max_speed),
                random.random_range(-max_speed..=max_speed),
                0.0,
            ),
            mass: 1.0,
            drag: 0.0,
            lifespan,
            health: lifespan,
        }
    }

    /// Is this particle dead?
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Apply a force to the particle
    pub fn apply_force(&mut self, force: Vec2) {
        let force = if self.mass > 0.0 {
            force / self.mass
        } else {
            force
        };

        self.acceleration += force.extend(0.0);
    }

    /// Updates the particle
    pub fn update(&mut self, dt: f32) {
        self.health -= dt;

        // TODO: fade the sprite by health / lifespan
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
