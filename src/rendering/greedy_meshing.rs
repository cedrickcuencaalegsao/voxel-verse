use crate::rendering::culling::{FACE_DIRECTIONS, is_face_visible};
use crate::utils::constants::{CHUNK_HEIGHT, CHUNK_SIZE};
use crate::world::block::BlockKind;
use crate::world::chunk::Chunk;

pub struct Quad {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub normal: [f32; 3],
    // pub uv: [f32; 4],
    pub block_kind: BlockKind,
}

/// Simple per-block-face mesh generation (one quad per visible face — not
/// actually greedy-merged yet, that's a performance follow-up, not a bug).
pub fn generate_chunk_quads(chunk: &Chunk) -> Vec<Quad> {
    let mut quads = Vec::new();
    let size = CHUNK_SIZE as i32;
    let height = CHUNK_HEIGHT as i32;

    for y in 0..height {
        for z in 0..size {
            for x in 0..size {
                let block = chunk.get_block(x as usize, y as usize, z as usize);
                if block.kind == BlockKind::Air {
                    continue;
                }
                for (dx, dy, dz, normal) in FACE_DIRECTIONS {
                    if is_face_visible(chunk, block, x + dx, y + dy, z + dz) {
                        quads.push(Quad {
                            position: [x as f32, y as f32, z as f32],
                            size: [1.0, 1.0],
                            normal,
                            // uv: [0.0, 0.0, 1.0, 1.0],
                            block_kind: block.kind,
                        });
                    }
                }
            }
        }
    }
    quads
}
