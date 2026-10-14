use super::meshing::remesh_chunks;
use crate::models::grass_model::{GrassModel, load_grass_model};
use crate::models::leaves_model::{LeavesModel, load_leaves_model};
use crate::models::sand_model::{SandModel, load_sand_model};
use crate::models::soil_model::{SoilModel, load_soil_model};
use crate::models::water_model::{WaterModel, load_water_model};
use crate::models::wood_model::{WoodModel, load_wood_model};
use crate::rendering::atlas::BlockAtlasPlugin;
use crate::world::world_manager::chunk_streaming_system;
use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockAtlasPlugin);
        app.init_resource::<GrassModel>()
            .init_resource::<LeavesModel>()
            .init_resource::<WoodModel>()
            .init_resource::<SoilModel>()
            .init_resource::<WaterModel>()
            .init_resource::<SandModel>();
        app.add_systems(
            Startup,
            (
                load_grass_model,
                load_leaves_model,
                load_wood_model,
                load_soil_model,
                load_water_model,
                load_sand_model,
            ),
        );
        app.add_systems(Update, remesh_chunks.after(chunk_streaming_system));
    }
}
