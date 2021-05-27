//! Creatures plugin

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::InspectableRegistry;

use crate::components::creatures::*;
use crate::components::physics::*;
use crate::components::*;
use crate::states::*;
use crate::systems::creatures::*;
use crate::systems::physics::*;

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            // per-frame update
            SystemSet::on_update(GameState::Game)
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
            // fixed (think) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(THINK_STEP as f64))
                .with_system(fly_think.system().label(CreaturesSystem::Think))
                .with_system(fish_think.system().label(CreaturesSystem::Think))
                .with_system(snake_think.system().label(CreaturesSystem::Think)),
        )
        .add_system_set(
            // fixed (physics) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
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
                ),
        );

        // register components for inspector
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);

        registry.register::<Creature>();
        registry.register::<Fly>();
        registry.register::<Firefly>();
        registry.register::<Fish>();
        registry.register::<Snake>();
    }
}
