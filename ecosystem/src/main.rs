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
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;
use num_traits::Float;

use components::physics::*;
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

    let mut app = App::new();

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
    .register_inspectable::<components::UiCamera>()
    .register_inspectable::<components::physics::Rigidbody>()
    .register_inspectable::<components::physics::Collider>()
    .register_inspectable::<components::physics::BoxCollider>()
    .register_inspectable::<components::physics::Oscillator>()
    .register_inspectable::<components::physics::Surface>()
    .register_inspectable::<components::physics::Fluid>()
    .register_inspectable::<components::particles::ParticleSystem>()
    .register_inspectable::<components::particles::Particle>()
    .register_inspectable::<components::creatures::Creature>()
    .register_inspectable::<components::creatures::Fly>()
    .register_inspectable::<components::creatures::Firefly>()
    .register_inspectable::<components::creatures::Fish>()
    .register_inspectable::<components::creatures::Snake>()
    .register_inspectable::<components::environment::Ground>()
    .register_inspectable::<components::environment::Water>()
    .register_inspectable::<components::environment::WaterCurrent>()
    .register_inspectable::<components::environment::Air>()
    .register_inspectable::<components::environment::Wind>();

    // plugins
    app.add_plugin(ParticleSystemPlugin);

    // events
    app.add_event::<ToggleDebugEvent>();

    // game states
    app.add_state(GameState::Game)
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(states::game::setup))
        .add_system_set(
            // fixed (physics) update
            // 1) all CreaturesSystem::Physics (before Physics, including repel)
            // 2) PhysicsSystem::Collisions (friction, drag, etc)
            // 3) PhysicsSystem::Update (move rigidbodies, oscillate, etc)
            // 4) CreaturesSystem::Bounds (rewind updates at borders, border repel)
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                // core physics
                .with_system(
                    physics_collisions
                        .label(Physics)
                        .label(PhysicsSystem::Collisions)
                        .before(PhysicsSystem::Update),
                )
                .with_system(physics_update.label(Physics).label(PhysicsSystem::Update))
                .with_system(
                    oscillator_update
                        .label(Physics)
                        .label(PhysicsSystem::Update),
                )
                // creaturue behaviors
                .with_system(fly_physics.label(CreaturesSystem::Physics).before(Physics))
                .with_system(fly_repel.label(CreaturesSystem::Physics).before(Physics))
                .with_system(fly_bounds.label(CreaturesSystem::Bounds).after(Physics))
                .with_system(fish_physics.label(CreaturesSystem::Physics).before(Physics))
                .with_system(fish_repel.label(CreaturesSystem::Physics).before(Physics))
                .with_system(fish_bounds.label(CreaturesSystem::Bounds).after(Physics))
                .with_system(
                    snake_physics
                        .label(CreaturesSystem::Physics)
                        .before(Physics),
                )
                .with_system(snake_repel.label(CreaturesSystem::Physics).before(Physics))
                .with_system(snake_bounds.label(CreaturesSystem::Bounds).after(Physics))
                .with_system(
                    water_current
                        .label(EnvironmentsSystem::Physics)
                        .before(Physics),
                )
                .with_system(wind.label(EnvironmentsSystem::Physics).before(Physics)),
        )
        .add_system_set(
            // per-frame update
            SystemSet::on_update(GameState::Game)
                .with_system(fly_update.label(CreaturesSystem::Update))
                .with_system(fish_update.label(CreaturesSystem::Update))
                .with_system(snake_update.label(CreaturesSystem::Update))
                .with_system(
                    creature_facing
                        .label(CreaturesSystem::UpdateAfter)
                        .after(CreaturesSystem::Update),
                ),
        )
        .add_system_set(SystemSet::on_exit(GameState::Game).with_system(states::game::teardown));

    // setup
    app.add_startup_system(setup)
        .add_startup_system(setup_debug);

    // debug
    app.add_system(debug_system).add_system(debug_ui);

    app.run();
}
