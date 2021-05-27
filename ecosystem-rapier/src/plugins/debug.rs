//! Debug plugin

use bevy::prelude::*;

use crate::events::debug::*;
use crate::systems::debug::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // events
        app.add_event::<ToggleDebugEvent>();

        // setup
        app.add_startup_system(setup_debug.system());

        // systems
        app.add_system(debug_system.system())
            .add_system(debug_ui.system());
    }
}
