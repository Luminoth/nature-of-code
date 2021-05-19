//! Nature of Code Ecosystem Project

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

use bevy::core::FixedTimestep;
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorParams, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use num_traits::Float;

use components::physics::*;
use components::*;
use events::debug::*;
use plugins::particles::*;
use resources::debug::*;
use resources::*;
use states::*;
use systems::creatures::*;
use systems::debug::*;
use systems::environment::*;
use systems::physics::*;

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
fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    _asset_server.watch_for_changes().unwrap();

    let mut random = Random::default();

    let simulation = SimulationParams {
        fly_count: random.normal_clamped::<f32>(10.0, 3.0, 3.0, 20.0) as usize,
        fish_count: random.normal_clamped::<f32>(4.0, 3.0, 2.0, 8.0) as usize,
        snake_count: random.normal_clamped::<f32>(2.0, 1.0, 1.0, 4.0) as usize,
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

    let mut app = App::build();

    // basic bevy
    app.insert_resource(WindowDescriptor {
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
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin);

    // rapier
    app.add_plugin(RapierPhysicsPlugin);

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
    .add_plugin(WorldInspectorPlugin::new());

    // plugins
    app.add_plugin(ParticleSystemPlugin);

    // events
    app.add_event::<ToggleDebugEvent>();

    // game states
    app.add_state(GameState::Game)
        .add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(states::game::setup.system()),
        )
        .add_system_set(
            // fixed (think) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(THINK_STEP as f64))
                .with_system(fly_think.system().label(CreaturesSystem::Think))
                .with_system(fish_think.system().label(CreaturesSystem::Think))
                .with_system(snake_think.system().label(CreaturesSystem::Think)),
        )
        .add_system_set(
            // fixed (physics) update
            // 1) all CreaturesSystem::Physics (before Physics, including repel)
            // 2) PhysicsSystem::Update (oscillate, etc)
            // 3) CreaturesSystem::Bounds (rewind updates at borders, border repel)
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                // creaturue behaviors
                .with_system(
                    fly_physics
                        .system()
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    fly_repel
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
                )
                .with_system(
                    water_current
                        .system()
                        .label(EnvironmentsSystem::Physics)
                        .before(Physics),
                )
                .with_system(
                    wind.system()
                        .label(EnvironmentsSystem::Physics)
                        .before(Physics),
                ),
        )
        .add_system_set(
            // per-frame update
            SystemSet::on_update(GameState::Game)
                .with_system(
                    oscillator_update
                        .system()
                        .label(Physics)
                        .label(PhysicsSystem::Update),
                )
                .with_system(fly_update.system().label(CreaturesSystem::Update))
                .with_system(fish_update.system().label(CreaturesSystem::Update))
                .with_system(snake_update.system().label(CreaturesSystem::Update))
                .with_system(
                    creature_facing
                        .system()
                        .label(CreaturesSystem::UpdateAfter)
                        .after(CreaturesSystem::Update),
                ),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(states::game::teardown.system()),
        );

    // physical stage
    app.add_stage_before(
        bevy_rapier2d::physics::TRANSFORM_SYNC_STAGE,
        "physical",
        SystemStage::single_threaded(),
    )
    .add_system_to_stage("physical", physical_update.system());

    // setup
    app.add_startup_system(setup.system())
        .add_startup_system(setup_debug.system());

    // debug
    app.add_system(debug_system.system())
        .add_system(debug_ui.system());

    // register components for inspector
    let mut registry = app
        .world_mut()
        .get_resource_or_insert_with(InspectableRegistry::default);

    registry.register::<components::MainCamera>();
    registry.register::<components::UiCamera>();
    registry.register::<components::physics::Physical>();
    registry.register::<components::physics::Oscillator>();
    registry.register::<components::particles::ParticleSystem>();
    registry.register::<components::particles::Particle>();
    registry.register::<components::creatures::Creature>();
    registry.register::<components::creatures::Fly>();
    registry.register::<components::creatures::Firefly>();
    registry.register::<components::creatures::Fish>();
    registry.register::<components::creatures::Snake>();
    registry.register::<components::environment::Ground>();
    registry.register::<components::environment::Water>();
    registry.register::<components::environment::WaterCurrent>();
    registry.register::<components::environment::Air>();
    registry.register::<components::environment::Wind>();

    app.run();
}
