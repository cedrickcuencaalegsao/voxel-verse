use super::chunk::Chunk;
use super::save::ChunkData;
use bevy::math::IVec3;
use std::path::PathBuf;

pub fn load_chunk(world_name: &str, pos: IVec3) -> Option<Chunk> {
    let path = PathBuf::from("world/saves")
        .join(world_name)
        .join("chunks")
        .join(format!("{}_{}_{}.bin", pos.x, pos.y, pos.z));
    if path.exists() {
        let bytes = std::fs::read(path).ok()?;
        let data: ChunkData = bincode::deserialize(&bytes).ok()?;
        data.to_chunk()
    } else {
        None
    }
}
