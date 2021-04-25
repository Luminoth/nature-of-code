//! Nature of Code Ecosystem Project

mod components;
mod events;
mod resources;
mod states;
mod systems;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use events::debug::*;
use resources::debug::*;
use resources::*;
use states::*;
use systems::creatures::*;
use systems::debug::*;
use systems::physics::*;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;

/// Misc setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(Random::default());
    commands.insert_resource(PerlinNoise::default());
}

/// Debug setup
fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugState::default());
}

/// Application entry
#[bevy_main]
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Ecosystem".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // events
        .add_event::<ToggleDebugEvent>()
        // game states
        .add_state(GameState::Game)
        .add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(states::game::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(
                    physics_collisions
                        .system()
                        .label("physics_collisions")
                        .before("physics_after"),
                )
                .with_system(physics_after.system().label("physics_after"))
                .with_system(
                    creature_after
                        .system()
                        .label("creature_after")
                        .before("physics_collisions"),
                )
                .with_system(
                    fly.system()
                        .before("creature_after")
                        .before("physics_collisions"),
                )
                .with_system(
                    fish.system()
                        .before("creature_after")
                        .before("physics_collisions"),
                )
                .with_system(
                    snake
                        .system()
                        .before("creature_after")
                        .before("physics_collisions"),
                ),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(states::game::teardown.system()),
        )
        // setup
        .add_startup_system(setup.system())
        .add_startup_system(setup_debug.system())
        // debug
        .add_system(debug_system.system())
        .add_system(fps_text_system.system())
        .run();
}
