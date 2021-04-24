//! Debugging systems

use bevy::diagnostic::*;
use bevy::prelude::*;

use crate::components::debug::*;
use crate::events::debug::*;
use crate::resources::debug::*;

/// Toggles debug on input
///
/// Sends the ToggleDebugEvent
pub fn debug_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut debug_state: ResMut<DebugState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_events: EventWriter<ToggleDebugEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        debug!("toggling debug ...");

        debug_state.enabled = !debug_state.enabled;

        #[allow(clippy::collapsible_else_if)]
        if debug_state.enabled {
            debug_state.fps_text_entity = Some(
                commands
                    .spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(5.0),
                                left: Val::Px(15.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "fps",
                            TextStyle {
                                font: asset_server.load("fonts/Roboto-Regular.ttf"),
                                font_size: 14.0,
                                color: Color::WHITE,
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(FpsText)
                    .id(),
            );
        } else {
            if let Some(fps_text) = debug_state.fps_text_entity.take() {
                commands.entity(fps_text).despawn_recursive();
            }
        }

        debug_events.send(ToggleDebugEvent);
    }
}

/// Handles FPS text
pub fn fps_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta().as_secs_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.sections[0].value = format!("{:.1} fps, {:.3} ms/frame", fps, frame_time * 1000.0);
    }
}
