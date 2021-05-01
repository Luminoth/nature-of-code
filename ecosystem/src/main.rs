//! Nature of Code Ecosystem Project

// bevy queries can produce a lot of this
#![allow(clippy::type_complexity)]

mod components;
mod events;
mod resources;
mod states;
mod systems;

use bevy::core::FixedTimestep;
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use num_traits::Float;

use components::physics::*;
use events::debug::*;
use resources::debug::*;
use resources::*;
use states::*;
use systems::creatures::*;
use systems::debug::*;
use systems::physics::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 576.0;

/// Clamps an ord between a min and a max
pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    std::cmp::min(max, std::cmp::max(min, v))
}

/// Clamps a float between a min and a max
pub fn clampf<F: Float>(v: F, min: F, max: F) -> F {
    Float::min(max, Float::max(min, v))
}

/// Misc setup
fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    _asset_server.watch_for_changes().unwrap();

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
    std::panic::set_hook(Box::new(|data| {
        error!(%data, "Unexpected panic!");
    }));

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
            // fixed (physics) update
            // 1) all CreaturesSystem::Physics (before Physics, including repel)
            // 2) PhysicsSystem::Collisions (friction, drag, etc)
            // 3) PhysicsSystem::Update (move rigidbodies)
            // 4) CreaturesSystem::Bounds (rewind updates at borders, border repel)
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                .with_system(
                    physics_collisions
                        .system()
                        .label(Physics)
                        .label(PhysicsSystem::Collisions)
                        .before(PhysicsSystem::Update),
                )
                .with_system(
                    physics_update
                        .system()
                        .label(Physics)
                        .label(PhysicsSystem::Update),
                )
                // creaturue behaviors
                .with_system(
                    fly_physics
                        .system()
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    fly_bounds
                        .system()
                        .label(CreaturesSystem::Bounds)
                        .after(Physics),
                )
                .with_system(
                    fish_physics
                        .system()
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    fish_repel
                        .system()
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    fish_bounds
                        .system()
                        .label(CreaturesSystem::Bounds)
                        .after(Physics),
                )
                .with_system(
                    snake_physics
                        .system()
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    snake_repel
                        .system()
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    snake_bounds
                        .system()
                        .label(CreaturesSystem::Bounds)
                        .after(Physics),
                ),
        )
        .add_system_set(
            // per-frame update
            SystemSet::on_update(GameState::Game)
                .with_system(physics_debug.system().label(PhysicsSystem::Debug))
                .with_system(fly_update.system().label(CreaturesSystem::Update))
                .with_system(fish_update.system().label(CreaturesSystem::Update))
                .with_system(snake_update.system().label(CreaturesSystem::Update)),
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
