//! ECS components

pub mod creatures;
pub mod environment;
pub mod particles;
pub mod physics;

use bevy_inspector_egui::Inspectable;

/// Main camera tag
#[derive(Debug, Inspectable, Default)]
pub struct MainCamera;

/// UI camera tag
#[derive(Debug, Inspectable, Default)]
pub struct UiCamera;
