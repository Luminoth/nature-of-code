//! Debugging systems

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::WorldInspectorParams;

use crate::events::debug::*;
use crate::resources::debug::*;

/// Debug setup
pub(crate) fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugState::default());
}

/// Toggles debug on input
///
/// Sends the ToggleDebugEvent
pub fn debug_system(
    mut inspector: ResMut<WorldInspectorParams>,
    mut debug_state: ResMut<DebugState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_events: EventWriter<ToggleDebugEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Grave) {
        debug!("toggling debug ...");

        debug_state.enabled = !debug_state.enabled;

        if !debug_state.enabled {
            inspector.enabled = false;
        }

        debug_events.send(ToggleDebugEvent);
    }
}

fn fps(diagnostics: &Diagnostics, dt: f64) -> (f64, f64) {
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.average() {
            fps = fps_avg;
        }
    }

    let mut frame_time = dt;
    if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time_diagnostic.average() {
            frame_time = frame_time_avg;
        }
    }

    (fps, frame_time)
}

/// Handles the debug UI
pub fn debug_ui(
    debug_state: ResMut<DebugState>,
    context: ResMut<EguiContext>,
    mut inspector: ResMut<WorldInspectorParams>,
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
) {
    if !debug_state.enabled {
        return;
    }

    let (fps, frame_time) = fps(&diagnostics, time.delta_seconds_f64());

    egui::Window::new("Debug").show(context.ctx(), |ui| {
        ui.vertical(|ui| {
            ui.label(format!(
                "{:.1} fps, {:.3} ms/frame",
                fps,
                frame_time * 1000.0
            ));

            if ui.button("Inspector").clicked() {
                inspector.enabled = !inspector.enabled;
            }

            // TODO: buttons to spawn creatures would be cool
            // but we need a way to say *where* to spawn them I think?
        });
    });
}
