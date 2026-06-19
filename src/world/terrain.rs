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
    /// Primary surface height noise
    perlin: Perlin,
    /// Second independent noise used for biome selection
    biome_noise: Perlin,
    /// 3D noise used for cave carving
    cave_noise: Perlin,
    /// 3D noise used for ore vein placement
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

    // ── surface height ────────────────────────────────────────────────────────

    /// Fractal Brownian Motion: layer `octaves` of Perlin at increasing
    /// frequencies and decreasing amplitudes.  Returns a value in [-1, 1].
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

    /// Smooth biome weight in [0, 1] from a raw noise value in [-1, 1].
    /// Values are mapped into three soft zones: plains / hills / mountains.
    fn biome_weights(&self, world_x: f64, world_z: f64) -> (f64, f64, f64) {
        let raw = self.biome_noise.get([world_x * 0.001, world_z * 0.001]);
        // Smoothstep-blend between three biomes based on noise value
        let t = (raw * 0.5 + 0.5).clamp(0.0, 1.0); // remap to [0,1]

        // plains dominates low t, mountains dominates high t
        let plains = smoothstep(0.0, 0.4, 1.0 - t);
        let hills = (1.0 - (t - 0.5).abs() * 4.0).clamp(0.0, 1.0);
        let mountains = smoothstep(0.6, 1.0, t);

        // normalise so the three weights sum to 1
        let total = plains + hills + mountains + f64::EPSILON;
        (plains / total, hills / total, mountains / total)
    }

    /// Surface height (in blocks) at the given world (x, z) position.
    pub fn height_at(&self, world_x: f64, world_z: f64) -> f64 {
        let (w_plains, w_hills, w_mountains) = self.biome_weights(world_x, world_z);

        // Plains: gentle undulation around sea level
        let plains_h = {
            let n = self.fbm(world_x, world_z, 4, 0.003);
            SEA_LEVEL as f64 + 4.0 + n * 8.0
        };

        // Hills: moderate relief
        let hills_h = {
            let n = self.fbm(world_x, world_z, 5, 0.005);
            SEA_LEVEL as f64 + 20.0 + n * 30.0
        };

        // Mountains: dramatic peaks with ridge noise
        let mountains_h = {
            // ridge noise: fold fBm to create sharp ridges
            let raw = self.fbm(world_x, world_z, 6, 0.008);
            let ridge = 1.0 - raw.abs() * 2.0; // peaks where noise ≈ 0
            SEA_LEVEL as f64 + 60.0 + ridge * 80.0
        };

        w_plains * plains_h + w_hills * hills_h + w_mountains * mountains_h
    }

    // ── cave carving ──────────────────────────────────────────────────────────

    /// Returns `true` if this position should be carved into a cave.
    /// Uses two independent 3D noise samples; a block is air when both are
    /// near zero, producing worm-like tunnels.
    fn is_cave(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        // Caves only carve through solid underground (not too close to surface)
        if world_y <= BEDROCK_Y + 1 || world_y > SEA_LEVEL - 5 {
            return false;
        }

        let scale = 0.05;
        let x = world_x as f64 * scale;
        let y = world_y as f64 * scale;
        let z = world_z as f64 * scale;

        let n1 = self.cave_noise.get([x, y, z]);
        // Offset second sample to get independent variation
        let n2 = self.cave_noise.get([x + 100.3, y + 57.1, z + 200.7]);

        // Cave threshold: both samples inside a narrow band around zero
        n1 * n1 + n2 * n2 < 0.02
    }

    // ── ore placement ─────────────────────────────────────────────────────────
    // Ore veins are modelled with 3-D Perlin noise.  Add CoalOre / IronOre /
    // DiamondOre variants to BlockKind (see block.rs) to enable them; until
    // then the function simply returns None and everything stays Stone.

    fn ore_at(&self, world_x: i32, world_y: i32, world_z: i32) -> Option<BlockKind> {
        let scale = 0.08;
        let n = self.ore_noise.get([
            world_x as f64 * scale,
            world_y as f64 * scale,
            world_z as f64 * scale,
        ]);
        let n_abs = n.abs();

        // Uncomment each arm once you add the matching BlockKind variant:
        if world_y < 16 && n_abs > 0.85 {
            // Some(BlockKind::DiamondOre)   // rare, deep only
            let _ = n_abs; // suppress unused-variable lint until variants exist
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

    // ── spawn guard ───────────────────────────────────────────────────────────

    /// Returns `true` for the flat core **and** the blend ring — both use
    /// `spawn_block_at()` so the height transitions cleanly.
    #[inline]
    fn is_spawn_area(world_x: i32, world_z: i32) -> bool {
        world_x * world_x + world_z * world_z <= SPAWN_BLEND_RADIUS * SPAWN_BLEND_RADIUS
    }

    /// Surface Y at (world_x, world_z) for spawning purposes.
    ///
    /// * Inside `SPAWN_RADIUS`        → fixed flat height (`SPAWN_FLAT_Y`).
    /// * `SPAWN_RADIUS..SPAWN_BLEND_RADIUS` → smooth smoothstep blend.
    /// * Outside `SPAWN_BLEND_RADIUS` → full natural terrain height.
    ///
    /// Always call this (not `height_at()`) from `spawn_player` so the player
    /// always lands on the actual ground block.
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

    // ── public API ────────────────────────────────────────────────────────────

    /// Determine block kind at given (world_x, world_y, world_z).
    pub fn block_at(&self, world_x: i32, world_y: i32, world_z: i32) -> Block {
        // ── bedrock floor ────────────────────────────────────────────────────
        if world_y <= BEDROCK_Y {
            return Block {
                kind: BlockKind::Bedrock,
            };
        }

        // ── spawn area: delegate to original simple logic ────────────────────
        if Self::is_spawn_area(world_x, world_z) {
            return self.spawn_block_at(world_x, world_y, world_z);
        }

        // ── natural terrain ──────────────────────────────────────────────────
        let surface_height = self.height_at(world_x as f64, world_z as f64) as i32;

        // Above surface → air
        if world_y > surface_height {
            return Block {
                kind: BlockKind::Air,
            };
        }

        // Cave carving (only underground, preserves surface)
        if world_y < surface_height && self.is_cave(world_x, world_y, world_z) {
            return Block {
                kind: BlockKind::Air,
            };
        }

        // Surface layer(s)
        if world_y == surface_height {
            // Beach / riverbank sand close to sea level
            if surface_height <= SEA_LEVEL + 2 {
                return Block {
                    kind: BlockKind::Sand,
                };
            }
            return Block {
                kind: BlockKind::Grass,
            };
        }

        // Sub-surface dirt band
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

        // Deep stone with possible ore veins
        if let Some(ore_kind) = self.ore_at(world_x, world_y, world_z) {
            return Block { kind: ore_kind };
        }

        Block {
            kind: BlockKind::Stone,
        }
    }

    /// Block generator for the spawn area (flat core + blend ring).
    ///
    /// Uses `spawn_height_at()` so the surface is always a flat platform at
    /// the centre and blends smoothly into natural terrain at the edges —
    /// no more stone walls shooting up through spawn.
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
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Classic smoothstep in [0, 1] for the range [edge0, edge1].
#[inline]
fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
