//! Particle system components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

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
    pool: Vec<Entity>,

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
            pool: Vec::with_capacity(capacity),
            live: Vec::with_capacity(capacity),
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
    pub fn spawn_particle(&mut self, commands: &mut Commands, transform: Transform) {
        // grow if we need to, this is pretty expensive
        if self.pool.is_empty() {
            debug!("Growing particle system {}", self.name);
            self.pool.reserve(self.capacity);
            self.spawn(commands);
        }

        let entity = self.pool.pop().unwrap();
        commands
            .entity(entity)
            .insert_bundle(ParticleBundle::new(transform, self.particle_lifespan))
            // TODO: this should be a child of the particle
            // but not sure how to remove it if we do that
            /*.insert_bundle(GeometryBuilder::build_as(
                &shapes::Ellipse {
                    radii: Vec2::splat(0.1),
                    ..Default::default()
                },
                ShapeColors::new(Color::RED),
                DrawMode::Fill(FillOptions::default()),
                transform,
            ))*/;
        self.live.push(entity);
    }

    /// Updates the particle system
    pub fn update(
        &mut self,
        commands: &mut Commands,
        time: &Time,
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
                    /*.remove_bundle::<ShapeBundle>()*/;

                self.pool.push(entity);
                self.live.remove(i);
            } else {
                i += 1;
            }
        }

        // spawn new particles last
        let now = time.seconds_since_startup();
        if now >= self.next_spawn {
            self.spawn_particle(commands, *transform);

            self.next_spawn = now + self.spawn_rate;
        }
    }
}

/// Particle component
#[derive(Debug, Inspectable)]
pub struct Particle {
    pub acceleration: Vec3,
    pub velocity: Vec3,

    pub lifespan: f32,
    pub health: f32,
}

impl Particle {
    /// Creates a new particle with the given lifespan
    pub fn new(lifespan: f32) -> Self {
        Self {
            acceleration: Vec3::default(),
            velocity: Vec3::default(),
            lifespan,
            health: lifespan,
        }
    }

    /// Is this particle dead?
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Updates the particle
    pub fn update(&mut self, dt: f32 /*, meshes: &mut Assets<Mesh>, mesh: &Handle<Mesh>*/) {
        self.health -= dt;

        // https://github.com/Nilirad/bevy_prototype_lyon/issues/96
        /*let mesh = meshes.get_mut(mesh).unwrap();
        let colors = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR).unwrap();
        let values = match colors {
            bevy::render::mesh::VertexAttributeValues::Float4(colors) => colors
                .iter()
                .map(|[r, g, b, _]| Color::rgba(*r, *g, *b, self.health / self.lifespan).into())
                .collect::<Vec<[f32; 4]>>(),
            _ => vec![],
        };
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, values);*/
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
