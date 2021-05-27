//! Environment plugin

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::InspectableRegistry;

use crate::components::environment::*;
use crate::components::physics::*;
use crate::states::*;
use crate::systems::environment::*;
use crate::systems::physics::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            // fixed (physics) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
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
        );

        // register components for inspector
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);

        registry.register::<Ground>();
        registry.register::<Water>();
        registry.register::<WaterCurrent>();
        registry.register::<Air>();
        registry.register::<Wind>();
    }
}
