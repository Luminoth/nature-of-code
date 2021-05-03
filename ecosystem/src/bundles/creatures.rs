//! Creature bundles

use bevy::prelude::*;

use crate::components::creatures::*;

use super::physics::*;

/// Fly bundle
#[derive(Default, Bundle)]
pub struct FlyBundle {
    pub fly: Fly,
    pub creature: Creature,

    #[bundle]
    pub physical: PhysicalBundle,
}

/// Fish bundle
#[derive(Default, Bundle)]
pub struct FishBundle {
    pub fish: Fish,
    pub creature: Creature,

    #[bundle]
    pub physical: PhysicalBundle,
}

/// Fish bundle
#[derive(Default, Bundle)]
pub struct SnakeBundle {
    pub snake: Snake,
    pub creature: Creature,

    #[bundle]
    pub physical: PhysicalBundle,
}
