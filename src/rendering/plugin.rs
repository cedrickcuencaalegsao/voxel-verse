use super::meshing::remesh_chunks;
use crate::world::world_manager::chunk_streaming_system;
use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, remesh_chunks.after(chunk_streaming_system));
    }
}
