//! Physics plugin

use bevy::prelude::*;
use bevy_inspector_egui::InspectableRegistry;

use crate::components::physics::*;
use crate::states::*;
use crate::systems::physics::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            // per-frame update
            SystemSet::on_update(GameState::Game).with_system(
                oscillator_update
                    .system()
                    .label(Physics)
                    .label(PhysicsSystem::Update),
            ),
        );

        // physical stage
        app.add_stage_before(
            bevy_rapier2d::physics::TRANSFORM_SYNC_STAGE,
            "physical",
            SystemStage::single_threaded(),
        )
        .add_system_to_stage("physical", physical_update.system());

        // register components for inspector
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);

        registry.register::<Physical>();
        registry.register::<Oscillator>();
    }
}
