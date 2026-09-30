use super::world_manager::chunk_streaming_system;
use crate::lobby::AppState;
use crate::world::block_registry::init_block_registry;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            chunk_streaming_system.run_if(in_state(AppState::Playing)),
        )
        .add_systems(Startup, init_block_registry);
    }
}