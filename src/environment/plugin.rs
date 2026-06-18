use super::daynight::{DayNightCycle, cycle_system, setup_sky};
use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DayNightCycle::default())
            .add_systems(Startup, setup_sky)
            .add_systems(Update, cycle_system);
    }
}
