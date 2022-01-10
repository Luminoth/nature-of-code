//! ECS components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Think step rate
/// 50hz
//pub const THINK_STEP: f32 = 0.02;

/// Main camera tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct MainCamera;

/// UI camera tag
#[derive(Debug, Default, Component, Inspectable)]
pub struct UiCamera;
