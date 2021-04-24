//! Creature resources

use bevy::prelude::*;

/// Fly materials
#[derive(Clone)]
pub struct FlyMaterials {
    pub material: Handle<ColorMaterial>,
}

/// Fish materials
#[derive(Clone)]
pub struct FishMaterials {
    pub material: Handle<ColorMaterial>,
}

/// Snake materials
#[derive(Clone)]
pub struct SnakeMaterials {
    pub material: Handle<ColorMaterial>,
}
