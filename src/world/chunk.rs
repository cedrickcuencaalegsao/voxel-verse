use super::block::Block;
use crate::utils::constants::{CHUNK_SIZE, CHUNK_VOLUME};
use bevy::prelude::*;

#[derive(Component)]
pub struct Chunk {
    pub position: IVec3,
    pub blocks: Box<[Block; CHUNK_VOLUME]>,
    pub needs_remesh: bool,
}

impl Chunk {
    pub fn new(position: IVec3, blocks: Box<[Block; CHUNK_VOLUME]>) -> Self {
        Self {
            position,
            blocks,
            needs_remesh: true,
        }
    }

    #[inline]
    pub fn block_index(x: usize, y: usize, z: usize) -> usize {
        y * (CHUNK_SIZE * CHUNK_SIZE) + z * CHUNK_SIZE + x
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[Self::block_index(x, y, z)]
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[Self::block_index(x, y, z)] = block;
        self.needs_remesh = true;
    }
}
