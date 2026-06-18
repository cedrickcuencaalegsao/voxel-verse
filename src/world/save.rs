use crate::utils::constants::CHUNK_VOLUME;
use crate::world::block::{Block, BlockKind};
use crate::world::chunk::Chunk;
use bevy::math::IVec3;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ChunkData {
    position: [i32; 3],
    blocks: Vec<u8>, // serialized block kinds as u8
}

impl ChunkData {
    fn from_chunk(chunk: &Chunk) -> Self {
        let blocks: Vec<u8> = chunk.blocks.iter().map(|b| b.kind as u8).collect();
        Self {
            position: [chunk.position.x, chunk.position.y, chunk.position.z],
            blocks,
        }
    }

    pub fn to_chunk(&self) -> Option<Chunk> {
        if self.blocks.len() != CHUNK_VOLUME {
            return None;
        }

        let mut blocks = Box::new([Block::default(); CHUNK_VOLUME]);
        for (i, &kind_byte) in self.blocks.iter().enumerate() {
            blocks[i] = Block {
                kind: BlockKind::try_from(kind_byte).ok()?,
            };
        }

        Some(Chunk::new(
            IVec3::new(self.position[0], self.position[1], self.position[2]),
            blocks,
        ))
    }
}

pub fn save_chunk(world_name: &str, chunk: &Chunk) -> std::io::Result<()> {
    let dir = PathBuf::from("world/saves").join(world_name).join("chunks");
    fs::create_dir_all(&dir)?;
    let filename = format!(
        "{}_{}_{}.bin",
        chunk.position.x, chunk.position.y, chunk.position.z
    );
    let path = dir.join(filename);
    let data = ChunkData::from_chunk(chunk);
    let bytes = bincode::serialize(&data).expect("Failed to serialize chunk");
    fs::write(path, bytes)
}
