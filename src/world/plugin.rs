use super::world_manager::{WorldManager, chunk_streaming_system};
use crate::world::block_registry::init_block_registry;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldManager::new(42)) // seed 42
            .add_systems(Update, chunk_streaming_system)
            .add_systems(Startup, init_block_registry);
    }
}
