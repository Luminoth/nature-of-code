//! ECS components

pub mod creatures;
pub mod environment;
pub mod particles;
pub mod physics;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Main camera tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct MainCamera;

/// UI camera tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct UiCamera;
