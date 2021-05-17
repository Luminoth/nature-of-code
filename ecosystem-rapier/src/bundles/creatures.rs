//! Creature bundles

use bevy::prelude::*;

use crate::components::creatures::*;
use crate::components::particles::*;

use super::physics::*;

/// Fly bundle
#[derive(Bundle)]
pub struct FlyBundle {
    pub fly: Fly,
    pub creature: Creature,

    #[bundle]
    pub physical: PhysicsBundle,
}

/// Firefly (Fly particles) bundle
#[derive(Default, Bundle)]
pub struct FireflyBundle {
    pub firefly: Firefly,

    pub particles: ParticleSystem,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Fish bundle
#[derive(Bundle)]
pub struct FishBundle {
    pub fish: Fish,
    pub creature: Creature,

    #[bundle]
    pub physical: PhysicsBundle,
}

/// Fish particles bundle
#[derive(Default, Bundle)]
pub struct FishParticlesBundle {
    pub particles: ParticleSystem,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Snake bundle
#[derive(Bundle)]
pub struct SnakeBundle {
    pub snake: Snake,
    pub creature: Creature,

    #[bundle]
    pub physical: PhysicsBundle,
}
