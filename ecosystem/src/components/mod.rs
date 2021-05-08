//! ECS components

pub mod creatures;
pub mod environment;
pub mod physics;

pub struct WorldBounds {
    pub width: f32,
    pub height: f32,
}

pub struct MainCamera;

pub struct UiCamera;
