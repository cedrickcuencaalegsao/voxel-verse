use super::block::{Block, BlockKind};
use super::chunk::Chunk;
use super::terrain::TerrainGenerator;
use super::tree::{oak_tree_blocks, pine_tree_blocks};
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
                chunk.set_block(x, y, z, generator.block_at(wx, wy, wz));
            }
        }
    }

    let border = 5i32;
    let cs = CHUNK_SIZE as i32;
    let ch = CHUNK_HEIGHT as i32;

    for lz in (-border)..(cs + border) {
        for lx in (-border)..(cs + border) {
            let wx = base_x + lx;
            let wz = base_z + lz;

            if !generator.should_spawn_tree(wx, wz) {
                continue;
            }

            let surface_y = generator.height_at(wx as f64, wz as f64) as i32;

            if surface_y <= 65 || surface_y > 190 {
                continue;
            }

            let (w_plains, w_hills, w_mountains) = generator.biome_weights(wx as f64, wz as f64);

            let trunk_base_y = surface_y + 1;

            let hash = (wx.wrapping_mul(1234567) ^ wz.wrapping_mul(7654321)).unsigned_abs();

            let tree_blocks = if w_mountains > 0.5 {
                let trunk_height = 8 + (hash % 5) as i32;
                pine_tree_blocks(trunk_height)
            } else if w_hills > w_plains {
                if hash % 2 == 0 {
                    let trunk_height = 5 + (hash % 3) as i32; // oak 5–7
                    oak_tree_blocks(trunk_height)
                } else {
                    let trunk_height = 7 + (hash % 4) as i32; // pine 7–10
                    pine_tree_blocks(trunk_height)
                }
            } else {
                let trunk_height = 4 + (hash % 3) as i32;
                oak_tree_blocks(trunk_height)
            };

            for (dx, dy, dz, kind) in tree_blocks {
                let block_wx = wx + dx;
                let block_wy = trunk_base_y + dy;
                let block_wz = wz + dz;

                let lx2 = block_wx - base_x;
                let ly2 = block_wy - base_y;
                let lz2 = block_wz - base_z;

                if lx2 < 0 || lx2 >= cs || ly2 < 0 || ly2 >= ch || lz2 < 0 || lz2 >= cs {
                    continue;
                }

                let existing = chunk.get_block(lx2 as usize, ly2 as usize, lz2 as usize);
                if existing.kind != BlockKind::Air {
                    continue;
                }

                chunk.set_block(lx2 as usize, ly2 as usize, lz2 as usize, Block { kind });
            }
        }
    }

    chunk
}
