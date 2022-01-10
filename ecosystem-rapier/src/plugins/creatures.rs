//! Creatures plugin

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::components::*;
use crate::states::*;
use crate::systems::creatures::*;
use crate::systems::physics::*;

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
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
        .add_system_set(
            // fixed (think) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(THINK_STEP as f64))
                .with_system(fly_think.label(CreaturesSystem::Think))
                .with_system(fish_think.label(CreaturesSystem::Think))
                .with_system(snake_think.label(CreaturesSystem::Think)),
        )
        .add_system_set(
            // fixed (physics) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
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
                .with_system(snake_bounds.label(CreaturesSystem::Bounds).after(Physics)),
        );

        // register components for inspector
        app.register_inspectable::<Creature>()
            .register_inspectable::<Fly>()
            .register_inspectable::<Firefly>()
            .register_inspectable::<Fish>()
            .register_inspectable::<Snake>();
    }
}
