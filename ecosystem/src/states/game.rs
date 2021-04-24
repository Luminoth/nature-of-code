//! Game state systems

use bevy::prelude::*;
use rand::Rng;

use crate::components::creatures::*;
use crate::components::*;
use crate::resources::creatures::*;
use crate::vec2_uniform;

/// Game setup
pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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
        let mut pos = vec2_uniform();
        pos.x *= hw - 5.0;
        pos.y *= hh - 5.0;
        pos.z = 2.0;
        info!("spawning fly at {}", pos);

        commands
            .spawn_bundle(SpriteBundle {
                material: fly_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(Physics {
                max_speed: 1.0,
                ..Default::default()
            })
            .insert(Fly::default());
    }

    // fish
    for _ in 0..rng.gen_range(3..6) {
        let mut pos = vec2_uniform();
        pos.x *= hw - 10.0;
        pos.y *= hh - 10.0;
        pos.z = 0.0;
        info!("spawning fish at {}", pos);

        commands
            .spawn_bundle(SpriteBundle {
                material: fish_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(Physics {
                max_speed: 0.5,
                ..Default::default()
            })
            .insert(Fish::default());
    }

    // snakes
    for _ in 0..rng.gen_range(1..3) {
        let mut pos = vec2_uniform();
        pos.x *= hw - 5.0;
        pos.y *= hh - 5.0;
        pos.z = 1.0;
        info!("spawning snake at {}", pos);

        commands
            .spawn_bundle(SpriteBundle {
                material: snake_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(Physics {
                max_speed: 1.5,
                ..Default::default()
            })
            .insert(Snake::default());
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
