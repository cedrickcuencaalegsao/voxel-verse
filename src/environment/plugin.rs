// use super::daynight::{DayNightCycle, cycle_system, setup_sky, sky_bodies_system};
// use bevy::prelude::*;

// pub struct EnvironmentPlugin;

// impl Plugin for EnvironmentPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(DayNightCycle::default())
//             .add_systems(Startup, setup_sky)
//             .add_systems(Update, (cycle_system, sky_bodies_system).chain());
//     }
// }

use super::daynight::{
    DayNightCycle, cycle_system, debug_lights_system, setup_sky, sky_bodies_system,
};
use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DayNightCycle::default())
            .add_systems(Startup, setup_sky)
            .add_systems(Update, (cycle_system, sky_bodies_system).chain())
            .add_systems(Update, debug_lights_system); // TEMPORARY: remove when fixed
    }
}