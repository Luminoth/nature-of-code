//! Creature resources

use bevy::prelude::*;

#[derive(Clone)]
pub struct FlyMaterials {
    pub material: Handle<ColorMaterial>,
}

#[derive(Clone)]
pub struct FishMaterials {
    pub material: Handle<ColorMaterial>,
}

#[derive(Clone)]
pub struct SnakeMaterials {
    pub material: Handle<ColorMaterial>,
}
