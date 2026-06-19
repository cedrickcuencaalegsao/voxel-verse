use super::atlas::setup_block_atlas;
use super::meshing::remesh_chunks;
use crate::rendering::atlas::BlockAtlasPlugin;
use crate::world::world_manager::chunk_streaming_system;
use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        // Builds the procedural atlas + materials once before the first
        // Update tick, so BlockAtlas is guaranteed to exist by the time
        // remesh_chunks first runs.
        app.add_plugins(BlockAtlasPlugin);
        app.add_systems(Startup, setup_block_atlas);
        app.add_systems(Update, remesh_chunks.after(chunk_streaming_system));
    }
}
