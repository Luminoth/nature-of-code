//! Game state systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use rand::Rng;

use crate::components::creatures::*;
use crate::resources::creatures::*;

/// Game setup
pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // physics
    commands.insert_resource(RapierConfiguration::default());

    // materials
    let fly_materials = FlyMaterials {
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
    };
    commands.insert_resource(fly_materials.clone());

    let fish_materials = FishMaterials {
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    };
    commands.insert_resource(fish_materials.clone());

    let snake_materials = SnakeMaterials {
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
    };
    commands.insert_resource(snake_materials.clone());

    // creatures

    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    // flies
    for _ in 0..rng.gen_range(1..5) {
        let pos = Vec2::new(
            rng.gen_range(-hw + 5.0..hw - 5.0),
            rng.gen_range(-hh + 5.0..hh - 5.0),
        );
        info!("spawning fly at {}", pos);

        commands
            .spawn_bundle(SpriteBundle {
                material: fly_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_kinematic().translation(pos.x, pos.y))
            .insert(Fly::default());
    }

    // fish
    for _ in 0..rng.gen_range(3..6) {
        let pos = Vec2::new(
            rng.gen_range(-hw + 10.0..hw - 10.0),
            rng.gen_range(-hh + 10.0..hh - 10.0),
        );
        info!("spawning fish at {}", pos);

        commands
            .spawn_bundle(SpriteBundle {
                material: fish_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_kinematic().translation(pos.x, pos.y))
            .insert(Fish::default());
    }

    // snakes
    for _ in 0..rng.gen_range(1..3) {
        let pos = Vec2::new(
            rng.gen_range(-hw + 5.0..hw - 5.0),
            rng.gen_range(-hh + 10.0..hh - 10.0),
        );
        info!("spawning snake at {}", pos);

        commands
            .spawn_bundle(SpriteBundle {
                material: snake_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 20.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_kinematic().translation(pos.x, pos.y))
            .insert(Snake::default());
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<RapierConfiguration>();
    commands.remove_resource::<ClearColor>();
}
