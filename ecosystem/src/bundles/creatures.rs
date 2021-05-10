//! Creature bundles

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::particles::*;

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

impl Default for FireflyBundle {
    fn default() -> Self {
        let particles = ParticleSystem::with_capacity("Firefly", 10);

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
