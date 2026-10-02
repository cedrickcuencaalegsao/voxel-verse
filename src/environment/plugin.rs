use super::daynight::{
    DayNightCycle, cycle_system, debug_lights_system, setup_sky, sky_bodies_system, remove_stray_lights
};
use bevy::light::DirectionalLightShadowMap;
use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(DayNightCycle::default())
            .add_systems(Startup, setup_sky)
            .add_systems(Update, (cycle_system, sky_bodies_system).chain())
            .add_systems(Update, (debug_lights_system, remove_stray_lights)); // TEMPORARY: remove when fixed
    }
}