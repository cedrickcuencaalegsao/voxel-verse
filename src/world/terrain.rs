use super::block::{Block, BlockKind};
use noise::{NoiseFn, Perlin};

// ── tuneable constants ────────────────────────────────────────────────────────

/// Radius (blocks) of the fully-flat spawn platform around (0, 0).
const SPAWN_RADIUS: i32 = 16;

/// Radius at which the flat spawn area finishes blending into natural terrain.
/// Between SPAWN_RADIUS and SPAWN_BLEND_RADIUS the height is lerped.
const SPAWN_BLEND_RADIUS: i32 = 40;

/// Fixed Y surface height of the spawn platform (a peaceful grassy plain).
const SPAWN_FLAT_Y: f64 = 66.0; // SEA_LEVEL(62) + 4 — a calm grassy plain

/// Y level treated as "sea" – sand/gravel appear near this height.
const SEA_LEVEL: i32 = 62;

/// Absolute bedrock floor.
const BEDROCK_Y: i32 = 0;

// ─────────────────────────────────────────────────────────────────────────────

pub struct TerrainGenerator {
    perlin: Perlin,
    biome_noise: Perlin,
    cave_noise: Perlin,
    ore_noise: Perlin,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            biome_noise: Perlin::new(seed.wrapping_add(1)),
            cave_noise: Perlin::new(seed.wrapping_add(2)),
            ore_noise: Perlin::new(seed.wrapping_add(3)),
        }
    }

    fn fbm(&self, x: f64, z: f64, octaves: u32, scale: f64) -> f64 {
        let mut value = 0.0_f64;
        let mut amplitude = 1.0_f64;
        let mut frequency = scale;
        let mut max_value = 0.0_f64; // used to normalise the result

        for _ in 0..octaves {
            value += self.perlin.get([x * frequency, z * frequency]) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value / max_value // normalised to [-1, 1]
    }

    pub fn biome_weights(&self, world_x: f64, world_z: f64) -> (f64, f64, f64) {
        let raw = self.biome_noise.get([world_x * 0.001, world_z * 0.001]);
        let t = (raw * 0.5 + 0.5).clamp(0.0, 1.0); // remap to [0,1]

        let plains = smoothstep(0.0, 0.4, 1.0 - t);
        let hills = (1.0 - (t - 0.5).abs() * 4.0).clamp(0.0, 1.0);
        let mountains = smoothstep(0.6, 1.0, t);

        let total = plains + hills + mountains + f64::EPSILON;
        (plains / total, hills / total, mountains / total)
    }

    pub fn height_at(&self, world_x: f64, world_z: f64) -> f64 {
        let (w_plains, w_hills, w_mountains) = self.biome_weights(world_x, world_z);

        let plains_h = {
            let n = self.fbm(world_x, world_z, 4, 0.003);
            SEA_LEVEL as f64 + 4.0 + n * 8.0
        };

        let hills_h = {
            let n = self.fbm(world_x, world_z, 5, 0.005);
            SEA_LEVEL as f64 + 20.0 + n * 30.0
        };

        let mountains_h = {
            let raw = self.fbm(world_x, world_z, 6, 0.008);
            let ridge = 1.0 - raw.abs() * 2.0; // peaks where noise ≈ 0
            SEA_LEVEL as f64 + 60.0 + ridge * 80.0
        };

        w_plains * plains_h + w_hills * hills_h + w_mountains * mountains_h
    }

    fn is_cave(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        if world_y <= BEDROCK_Y + 1 || world_y > SEA_LEVEL - 5 {
            return false;
        }

        let scale = 0.05;
        let x = world_x as f64 * scale;
        let y = world_y as f64 * scale;
        let z = world_z as f64 * scale;

        let n1 = self.cave_noise.get([x, y, z]);
        let n2 = self.cave_noise.get([x + 100.3, y + 57.1, z + 200.7]);

        n1 * n1 + n2 * n2 < 0.02
    }

    fn ore_at(&self, world_x: i32, world_y: i32, world_z: i32) -> Option<BlockKind> {
        let scale = 0.08;
        let n = self.ore_noise.get([
            world_x as f64 * scale,
            world_y as f64 * scale,
            world_z as f64 * scale,
        ]);
        let n_abs = n.abs();

        if world_y < 16 && n_abs > 0.85 {
            let _ = n_abs;
            None
        } else if world_y < 32 && n_abs > 0.80 {
            // Some(BlockKind::IronOre)
            None
        } else if world_y < 60 && n_abs > 0.75 {
            // Some(BlockKind::CoalOre)
            None
        } else {
            None
        }
    }

    #[inline]
    fn is_spawn_area(world_x: i32, world_z: i32) -> bool {
        world_x * world_x + world_z * world_z <= SPAWN_BLEND_RADIUS * SPAWN_BLEND_RADIUS
    }

    pub fn spawn_height_at(&self, world_x: f64, world_z: f64) -> f64 {
        let dist = (world_x * world_x + world_z * world_z).sqrt();
        let natural = self.height_at(world_x, world_z);

        if dist <= SPAWN_RADIUS as f64 {
            SPAWN_FLAT_Y
        } else if dist >= SPAWN_BLEND_RADIUS as f64 {
            natural
        } else {
            // smoothstep blend: 0 at SPAWN_RADIUS, 1 at SPAWN_BLEND_RADIUS
            let t =
                (dist - SPAWN_RADIUS as f64) / (SPAWN_BLEND_RADIUS as f64 - SPAWN_RADIUS as f64);
            let s = t * t * (3.0 - 2.0 * t);
            SPAWN_FLAT_Y * (1.0 - s) + natural * s
        }
    }

    pub fn block_at(&self, world_x: i32, world_y: i32, world_z: i32) -> Block {
        if world_y <= BEDROCK_Y {
            return Block {
                kind: BlockKind::Bedrock,
            };
        }

        if Self::is_spawn_area(world_x, world_z) {
            return self.spawn_block_at(world_x, world_y, world_z);
        }

        let surface_height = self.height_at(world_x as f64, world_z as f64) as i32;

        if world_y > surface_height {
            return Block {
                kind: BlockKind::Air,
            };
        }

        if world_y < surface_height && self.is_cave(world_x, world_y, world_z) {
            return Block {
                kind: BlockKind::Air,
            };
        }

        if world_y == surface_height {
            if surface_height <= SEA_LEVEL + 2 {
                return Block {
                    kind: BlockKind::Sand,
                };
            }
            return Block {
                kind: BlockKind::Grass,
            };
        }

        let depth = surface_height - world_y;
        if depth <= 4 {
            if surface_height <= SEA_LEVEL + 2 {
                return Block {
                    kind: BlockKind::Sand,
                };
            }
            return Block {
                kind: BlockKind::Dirt,
            };
        }

        if let Some(ore_kind) = self.ore_at(world_x, world_y, world_z) {
            return Block { kind: ore_kind };
        }

        Block {
            kind: BlockKind::Stone,
        }
    }

    fn spawn_block_at(&self, world_x: i32, world_y: i32, world_z: i32) -> Block {
        if world_y == 0 {
            return Block {
                kind: BlockKind::Bedrock,
            };
        }

        let surface_height = self.spawn_height_at(world_x as f64, world_z as f64) as i32;

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

    pub fn should_spawn_tree(&self, world_x: i32, world_z: i32) -> bool {
        let scale = 0.15;
        let n = self.biome_noise.get([
            world_x as f64 * scale + 500.0,
            world_z as f64 * scale + 500.0,
        ]);

        if n < 0.65 {
            return false;
        }

        let hash = (world_x.wrapping_mul(374761393) ^ world_z.wrapping_mul(668265263)) as u32;
        hash % 12 == 0
    }
}

#[inline]
fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
