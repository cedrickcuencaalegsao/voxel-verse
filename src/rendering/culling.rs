use crate::utils::constants::{CHUNK_HEIGHT, CHUNK_SIZE};
use crate::world::block::Block;
use crate::world::chunk::Chunk;

/// The 6 axis-aligned face directions a block can expose, paired with their outward normal.
pub const FACE_DIRECTIONS: [(i32, i32, i32, [f32; 3]); 6] = [
    (1, 0, 0, [1.0, 0.0, 0.0]),
    (-1, 0, 0, [-1.0, 0.0, 0.0]),
    (0, 1, 0, [0.0, 1.0, 0.0]),
    (0, -1, 0, [0.0, -1.0, 0.0]),
    (0, 0, 1, [0.0, 0.0, 1.0]),
    (0, 0, -1, [0.0, 0.0, -1.0]),
];

/// Whether `block`'s face pointing at (x, y, z) should be drawn, given the
/// neighbour found there. Out-of-chunk neighbours are treated as visible
/// (air) for now since we don't have access to adjacent chunks yet — that
/// just means border faces always render, which is harmless over-draw, not
/// a missing-face bug.
pub fn is_face_visible(chunk: &Chunk, block: Block, x: i32, y: i32, z: i32) -> bool {
    let size = CHUNK_SIZE as i32;
    let height = CHUNK_HEIGHT as i32;
    if x < 0 || x >= size || y < 0 || y >= height || z < 0 || z >= size {
        return true;
    }
    let neighbour = chunk.get_block(x as usize, y as usize, z as usize);
    !neighbour.is_solid() || (neighbour.is_transparent() && block.is_transparent())
}
