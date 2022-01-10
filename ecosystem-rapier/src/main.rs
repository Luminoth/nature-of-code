//! Nature of Code Ecosystem Project (Rapier Physics)

// bevy queries can produce a lot of this
#![allow(clippy::type_complexity)]

mod bundles;
mod components;
mod events;
mod plugins;
mod resources;
mod states;
mod systems;
mod util;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use num_traits::Float;

use plugins::creatures::*;
use plugins::debug::*;
use plugins::environment::*;
use plugins::particles::*;
use plugins::physics::*;
use resources::*;
use states::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 576.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;
const WORLD_SIZE: f32 = 10.0;

/// Clamps an ord between a min and a max
pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    std::cmp::min(max, std::cmp::max(min, v))
}

/// Clamps a float between a min and a max
pub fn clampf<F: Float>(v: F, min: F, max: F) -> F {
    Float::min(max, Float::max(min, v))
}

/// Misc setup
fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    #[cfg(debug_assertions)]
    _asset_server.watch_for_changes().unwrap();

    rapier_config.gravity = Vector::default();

    let mut random = Random::default();

    let simulation = SimulationParams {
        fly_count: random.normal_clamped::<f32>(10.0, 3.0, 3.0, 20.0) as usize,
        fish_count: random.normal_clamped::<f32>(4.0, 3.0, 2.0, 8.0) as usize,
        snake_count: random.normal_clamped::<f32>(2.0, 1.0, 1.0, 4.0) as usize,

        enable_current: false,
        enable_wind: false,
    };

    let world_bounds = WorldBounds {
        width: WORLD_SIZE * ASPECT_RATIO,
        height: WORLD_SIZE,
    };

    commands.insert_resource(random);
    commands.insert_resource(PerlinNoise::default());
    commands.insert_resource(simulation);
    commands.insert_resource(world_bounds);
}

/// Application entry
#[bevy_main]
fn main() {
    std::panic::set_hook(Box::new(|data| {
        error!(%data, "Unexpected panic!");
    }));

    let mut app = App::new();

    // basic bevy
    app.insert_resource(WindowDescriptor {
        title: "Ecosystem - Rapier".to_owned(),
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
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin);

    // rapier
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

    // prototype lyon
    app.add_plugin(ShapePlugin);

    // egui
    app.insert_resource(EguiSettings { scale_factor: 0.8 })
        .add_plugin(EguiPlugin);

    // inspector
    app.insert_resource(WorldInspectorParams {
        enabled: false,
        despawnable_entities: true,
        ..Default::default()
    })
    .add_plugin(WorldInspectorPlugin::new())
    .register_inspectable::<components::MainCamera>()
    .register_inspectable::<components::UiCamera>();

    // plugins
    app.add_plugin(DebugPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ParticleSystemPlugin)
        .add_plugin(EnvironmentPlugin)
        .add_plugin(CreaturesPlugin);

    // game states
    app.add_state(GameState::Game)
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(states::game::setup))
        .add_system_set(SystemSet::on_exit(GameState::Game).with_system(states::game::teardown));

    // setup
    app.add_startup_system(setup);

    app.run();
}
