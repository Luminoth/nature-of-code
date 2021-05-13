//! Creature bundles

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::particles::*;
use crate::resources::*;

use super::physics::*;

/// Fly bundle
#[derive(Default, Bundle)]
pub struct FlyBundle {
    pub fly: Fly,
    pub creature: Creature,

    #[bundle]
    pub physical: DynamicPhysicsBundle,
}

/// Firefly bundle
///
/// Requires an existing FlyBundle
#[derive(Bundle)]
pub struct FireflyBundle {
    pub firefly: Firefly,
    pub particles: ParticleSystem,
}

impl FireflyBundle {
    pub fn new(random: &mut Random, materials: &mut Assets<ColorMaterial>) -> Self {
        // TODO: we can calculate the required capacity
        // from the spawn rate and lifespan
        let mut particles =
            ParticleSystem::with_capacity("Firefly", materials.add(FIREFLY_COLOR.into()), 20);
        particles.spawn_rate = 0.05;
        particles.particle_lifespan = 0.5;
        particles.max_speed = random.normal(0.5, 0.1);

        Self {
            firefly: Firefly::default(),
            particles,
        }
    }
}

/// Fish bundle
#[derive(Default, Bundle)]
pub struct FishBundle {
    pub fish: Fish,
    pub creature: Creature,

    #[bundle]
    pub physical: DynamicPhysicsBundle,
}

/// Snake bundle
#[derive(Default, Bundle)]
pub struct SnakeBundle {
    pub snake: Snake,
    pub creature: Creature,

    #[bundle]
    pub physical: DynamicPhysicsBundle,
}
