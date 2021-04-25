//! Game state systems

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::creatures::*;
use crate::components::environment::*;
use crate::components::*;
//use crate::resources::creatures::*;
use crate::resources::*;
use crate::vec2_uniform;

/// Game setup
pub fn setup(
    mut commands: Commands,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    mut random: ResMut<Random>,
    windows: Res<Windows>,
) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // materials
    /*let fly_materials = FlyMaterials {
        material: materials.add(Color::BLUE.into()),
    };
    commands.insert_resource(fly_materials.clone());

    let fish_materials = FishMaterials {
        material: materials.add(Color::RED.into()),
    };
    commands.insert_resource(fish_materials.clone());

    let snake_materials = SnakeMaterials {
        material: materials.add(Color::GREEN.into()),
    };
    commands.insert_resource(snake_materials.clone());*/

    let window = windows.get_primary().unwrap();
    let qw = window.width() as f32 / 4.0;
    let hw = window.width() as f32 / 2.0;
    let hh = window.height() as f32 / 2.0;

    // environment

    // ground
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                width: qw,
                height: window.height(),
                origin: shapes::RectangleOrigin::TopLeft,
            },
            ShapeColors::new(Color::rgba(0.0, 0.5, 0.0, 0.5)),
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(qw, hh, 10.0)),
        ))
        .insert(Surface::default())
        .insert(Ground::default());

    // water
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                width: window.width() * 0.75,
                height: window.height(),
                origin: shapes::RectangleOrigin::TopLeft,
            },
            ShapeColors::new(Color::rgba(0.18, 0.55, 0.34, 0.5)),
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(-hw, hh, 5.0)),
        ))
        .insert(Fluid::default())
        .insert(Water::default());

    // creatures

    // flies
    let shape = shapes::Ellipse {
        radii: Vec2::new(2.0, 2.0),
        ..Default::default()
    };

    for _ in 0..random.normal_clamped::<f32>(5.0, 1.0, 3.0, 6.0) as u32 {
        let mut pos = Vec3::from((vec2_uniform(&mut *random), 100.0));
        pos.x *= hw - 5.0;
        pos.y *= hh - 5.0;
        info!("spawning fly at {}", pos);

        commands
            /*.spawn_bundle(SpriteBundle {
                material: fly_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })*/
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(Color::WHITE),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(pos),
            ))
            .insert(Rigidbody {
                max_speed: 1.25,
                ..Default::default()
            })
            .insert(Creature::default())
            .insert(Fly::default());
    }

    // fish
    let shape = shapes::Ellipse {
        radii: Vec2::new(10.0, 10.0),
        ..Default::default()
    };

    for _ in 0..random.normal_clamped::<f32>(4.0, 3.0, 2.0, 8.0) as u32 {
        let mut pos = Vec3::from((vec2_uniform(&mut *random), 0.0));
        pos.x *= hw - 10.0;
        pos.y *= hh - 10.0;
        info!("spawning fish at {}", pos);

        commands
            /*.spawn_bundle(SpriteBundle {
                material: fish_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })*/
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(Color::SALMON),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(pos),
            ))
            .insert(Rigidbody {
                max_speed: 0.5,
                ..Default::default()
            })
            .insert(Creature::default())
            .insert(Fish::new(2.0));
    }

    // snakes
    let shape = shapes::Ellipse {
        radii: Vec2::new(5.0, 5.0),
        ..Default::default()
    };

    for _ in 0..random.normal_clamped::<f32>(2.0, 1.0, 1.0, 4.0) as u32 {
        let mut pos = Vec3::from((vec2_uniform(&mut *random), 20.0));
        pos.x *= hw - 5.0;
        pos.y *= hh - 5.0;
        info!("spawning snake at {}", pos);

        commands
            /*.spawn_bundle(SpriteBundle {
                material: snake_materials.material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })*/
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::new(Color::MAROON),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(pos),
            ))
            .insert(Rigidbody {
                max_speed: 1.0,
                ..Default::default()
            })
            .insert(Creature::default())
            .insert(Snake::new(2.0));
    }
}

/// Game teardown
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
