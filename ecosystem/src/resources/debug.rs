//! Debug resources

use bevy::prelude::*;

/// Holds whatever debug state we need to keep around
#[derive(Default)]
pub struct DebugState {
    pub enabled: bool,

    pub fps_text_entity: Option<Entity>,
}
