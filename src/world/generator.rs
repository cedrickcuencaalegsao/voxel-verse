use super::block::Block;
use super::chunk::Chunk;
use super::terrain::TerrainGenerator;
use crate::utils::constants::{CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_VOLUME};
use bevy::prelude::*;

pub fn generate_chunk(chunk_pos: IVec3, generator: &TerrainGenerator) -> Chunk {
    let base_x = chunk_pos.x * CHUNK_SIZE as i32;
    let base_y = chunk_pos.y * CHUNK_HEIGHT as i32;
    let base_z = chunk_pos.z * CHUNK_SIZE as i32;
    let mut chunk = Chunk::new(chunk_pos, Box::new([Block::default(); CHUNK_VOLUME]));

    for y in 0..CHUNK_HEIGHT {
        let wy = base_y + y as i32;
        for z in 0..CHUNK_SIZE {
            let wz = base_z + z as i32;
            for x in 0..CHUNK_SIZE {
                let wx = base_x + x as i32;
                let block = generator.block_at(wx, wy, wz);
                chunk.set_block(x, y, z, block);
            }
        }
    }

    chunk
}
