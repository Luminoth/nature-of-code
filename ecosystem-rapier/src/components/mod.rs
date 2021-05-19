//! ECS components

pub mod creatures;
pub mod environment;
pub mod particles;
pub mod physics;

use bevy_inspector_egui::Inspectable;

/// Think step rate
/// 50hz
pub const THINK_STEP: f32 = 0.02;

/// Main camera tag
#[derive(Debug, Inspectable, Default)]
pub struct MainCamera;

/// UI camera tag
#[derive(Debug, Inspectable, Default)]
pub struct UiCamera;
