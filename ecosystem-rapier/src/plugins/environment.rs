//! Environment plugin

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use crate::components::environment::*;
use crate::components::physics::*;
use crate::states::*;
use crate::systems::environment::*;
use crate::systems::physics::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            // fixed (physics) update
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(PHYSICS_STEP as f64))
                .with_system(
                    water_current
                        .label(EnvironmentsSystem::Physics)
                        .before(Physics),
                )
                .with_system(wind.label(EnvironmentsSystem::Physics).before(Physics)),
        );

        // register components for inspector
        app.register_inspectable::<Ground>()
            .register_inspectable::<Water>()
            .register_inspectable::<WaterCurrent>()
            .register_inspectable::<Air>()
            .register_inspectable::<Wind>();
    }
}
