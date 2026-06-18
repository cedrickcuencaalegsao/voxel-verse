use super::block::{Block, BlockKind};
use noise::{NoiseFn, Perlin};

pub struct TerrainGenerator {
    perlin: Perlin,
    // additional noise for biomes, etc.
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
        }
    }

    /// Height at world coordinates (x,z)
    pub fn height_at(&self, world_x: f64, world_z: f64) -> f64 {
        let scale = 0.005;
        let noise = self.perlin.get([world_x * scale, world_z * scale]);
        // Map noise [-1,1] to height range [40, 200]
        120.0 * (noise * 0.5 + 0.5) + 40.0
    }

    /// Determine block kind at given (world_x, world_y, world_z)
    pub fn block_at(&self, world_x: i32, world_y: i32, world_z: i32) -> Block {
        if world_y < 0 {
            return Block {
                kind: BlockKind::Bedrock,
            };
        }
        if world_y == 0 {
            return Block {
                kind: BlockKind::Bedrock,
            };
        }

        let surface_height = self.height_at(world_x as f64, world_z as f64) as i32;
        if world_y > surface_height {
            Block {
                kind: BlockKind::Air,
            }
        } else if world_y == surface_height {
            Block {
                kind: BlockKind::Grass,
            }
        } else if world_y > surface_height - 5 {
            Block {
                kind: BlockKind::Dirt,
            }
        } else {
            Block {
                kind: BlockKind::Stone,
            }
        }
    }
}
