//! Nature of Code Ecosystem Project

mod components;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;
use noise::{Perlin, Seedable};
use rand::{random, Rng};

use states::*;
use systems::creatures::*;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;

pub fn vec2_random(xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(xmin..xmax), rng.gen_range(ymin..ymax), 0.0)
}

/// Misc setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(Perlin::new().set_seed(random()));
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
        // game states
        .add_state(GameState::Game)
        .add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(states::game::setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(fly.system())
                .with_system(fish.system())
                .with_system(snake.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(states::game::teardown.system()),
        )
        // setup
        .add_startup_system(setup.system())
        .run();
}
