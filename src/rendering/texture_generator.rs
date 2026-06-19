//! Procedural pixel-art block texture generators.
//!
//! Every public function returns a `Vec<u8>` of raw RGBA data for a
//! `TILE_SIZE × TILE_SIZE` tile.  No external files, no PNG assets,
//! no third-party RNG crates are used anywhere in this module.
//!
//! Design notes
//! ────────────
//! * Noise is produced by the splitmix64 finalizer — fast, dependency-free,
//!   and deterministic across platforms (important for Apple Silicon).
//! * 4×4 cluster noise mirrors Minecraft's chunky pixel-art aesthetic.
//! * Each generator has its own seed constant so textures never correlate.
//! * `generate_all_tiles()` returns tiles in `TileIndex` discriminant order;
//!   keep that ordering in sync with `atlas.rs`.

pub const TILE_SIZE: u32 = 64;
const CLUSTER: u32 = 4; // pixels per noise cluster edge

// ── Deterministic RNG ─────────────────────────────────────────────────────────

/// Splitmix64 finalizer — bijective, avalanche-quality, branch-free.
#[inline(always)]
fn hash64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^= x >> 31;
    x
}

#[inline(always)]
fn cluster_noise(px: u32, py: u32, seed: u64) -> u8 {
    let x = ((px / CLUSTER) as u64).wrapping_mul(0x9e3779b97f4a7c15);
    let y = ((py / CLUSTER) as u64).wrapping_mul(0x6c62272e07bb0142);

    let h = hash64(seed ^ x ^ y);
    (h >> 56) as u8
}

#[inline(always)]
fn pixel_noise(px: u32, py: u32, seed: u64) -> u8 {
    let x = (px as u64).wrapping_mul(0x9e3779b97f4a7c15);
    let y = (py as u64).wrapping_mul(0x6c62272e07bb0142);

    let h = hash64(seed ^ x ^ y);
    (h >> 56) as u8
}

// ── Colour helpers ────────────────────────────────────────────────────────────

#[inline(always)]
fn sat(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

/// Shift every channel of `base` by the same signed delta derived from
/// `noise_byte`.  `range` controls the maximum deviation (±range/2).
/// Returns a full RGBA pixel with alpha = 255.
#[inline(always)]
fn vary(base: [u8; 3], noise_byte: u8, range: i32) -> [u8; 4] {
    let d = noise_byte as i32 * range / 255 - range / 2;
    [
        sat(base[0] as i32 + d),
        sat(base[1] as i32 + d),
        sat(base[2] as i32 + d),
        255,
    ]
}

// ── Internal tile utilities ───────────────────────────────────────────────────

fn blank_tile() -> Vec<u8> {
    vec![0u8; (TILE_SIZE * TILE_SIZE * 4) as usize]
}

#[inline(always)]
fn put(buf: &mut [u8], x: u32, y: u32, pixel: [u8; 4]) {
    let i = ((y * TILE_SIZE + x) * 4) as usize;
    buf[i..i + 4].copy_from_slice(&pixel);
}

// ── Block texture generators ──────────────────────────────────────────────────

/// Stone — uniform grey with blocky noise patches.
pub fn gen_stone() -> Vec<u8> {
    const S: u64 = 0xAAAA_1111_BBBB_2222;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            put(
                &mut buf,
                x,
                y,
                vary([120, 120, 120], cluster_noise(x, y, S), 40),
            );
        }
    }
    buf
}

/// Dirt — warm brown with slightly darker cluster patches.
pub fn gen_dirt() -> Vec<u8> {
    const S: u64 = 0xCCCC_3333_DDDD_4444;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            put(
                &mut buf,
                x,
                y,
                vary([110, 70, 40], cluster_noise(x, y, S), 35),
            );
        }
    }
    buf
}

/// Grass top — bright green with subtle cluster variation.
pub fn gen_grass_top() -> Vec<u8> {
    const S: u64 = 0xEEEE_5555_FFFF_6666;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            put(
                &mut buf,
                x,
                y,
                vary([34, 180, 34], cluster_noise(x, y, S), 30),
            );
        }
    }
    buf
}

/// Grass side — a green cap (~1/6 of height) blending into dirt below.
/// The dirt seed matches `gen_dirt` so the seam looks natural.
pub fn gen_grass_side() -> Vec<u8> {
    const S_G: u64 = 0xABCD_1234_EF01_5678;
    const S_D: u64 = 0xCCCC_3333_DDDD_4444; // intentionally matches gen_dirt
    let cap = TILE_SIZE / 6; // ≈ 10–11 px green band at the top
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let pixel = if y < cap {
                vary([34, 180, 34], cluster_noise(x, y, S_G), 25)
            } else {
                vary([110, 70, 40], cluster_noise(x, y, S_D), 30)
            };
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}

/// Sand — warm beige with layered coarse + fine grain.
pub fn gen_sand() -> Vec<u8> {
    const SC: u64 = 0x1122_AABB_3344_CCDD;
    const SF: u64 = 0x5566_EEFF_7788_0011;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, SC) / 2 + pixel_noise(x, y, SF) / 2;
            put(&mut buf, x, y, vary([220, 210, 120], v, 25));
        }
    }
    buf
}

/// Water — deep blue with horizontal sine-wave brightness bands.
/// Alpha is 200 (≈78 %) so underwater geometry shows through.
pub fn gen_water() -> Vec<u8> {
    const S: u64 = 0xBEEF_CAFE_DEAD_BEEF;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let wave = ((y as f32 / 8.0 * std::f32::consts::TAU).sin() * 18.0) as i32;
            let n = cluster_noise(x, y, S) as i32 * 14 / 255 - 7;
            let pixel = [
                sat(30 + wave / 5 + n),
                sat(80 + wave / 3 + n),
                sat(220 + wave + n),
                200,
            ];
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}

/// Wood (log side) — warm brown with wavy vertical grain lines.
/// The grain wobble is driven by a separate per-row seed so lines aren't
/// perfectly straight, matching Minecraft's log appearance.
pub fn gen_wood() -> Vec<u8> {
    const SG: u64 = 0x1357_2468_9ABC_DEF0; // grain wobble per row
    const SB: u64 = 0xAABB_CCDD_EEFF_0011; // base colour cluster noise
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            // Shift x slightly per row to create a wavy grain effect.
            let wobble = pixel_noise(0, y, SG) as i32 / 16 - 8;
            let grain_x = (x as i32 + wobble).rem_euclid(TILE_SIZE as i32) as u32;
            let on_grain = (grain_x % 8) < 2;
            let dark = if on_grain { 28i32 } else { 0 };
            let base = [sat(120 - dark), sat(80 - dark), sat(40 - dark)];
            put(&mut buf, x, y, vary(base, cluster_noise(x, y, SB), 18));
        }
    }
    buf
}

/// Leaves — two-tone green: dark irregular patches on a lighter base.
pub fn gen_leaves() -> Vec<u8> {
    const S: u64 = 0xFEED_FACE_C0FF_EEEE;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, S);
            let base = if v < 90 {
                [20u8, 110, 20]
            } else {
                [34, 155, 34]
            };
            put(&mut buf, x, y, vary(base, v, 20));
        }
    }
    buf
}

/// Bedrock — near-black with coarse + fine irregular veins.
pub fn gen_bedrock() -> Vec<u8> {
    const SC: u64 = 0x0000_DEAD_BEEF_0000;
    const SF: u64 = 0x1111_2222_3333_4444;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, SC) / 2 + pixel_noise(x, y, SF) / 2;
            put(&mut buf, x, y, vary([40, 40, 40], v, 30));
        }
    }
    buf
}

// ── Atlas entry point ─────────────────────────────────────────────────────────

/// Returns all tile RGBA buffers in canonical `TileIndex` discriminant order.
///
/// ```text
/// Index  Tile
/// ─────  ────────────
///   0    Stone
///   1    Dirt
///   2    GrassTop
///   3    GrassSide
///   4    Sand
///   5    Water
///   6    Wood
///   7    Leaves
///   8    Bedrock
/// ```
///
/// **This ordering must stay in sync with `TileIndex` in `atlas.rs`.**
pub fn generate_all_tiles() -> [Vec<u8>; 9] {
    [
        gen_stone(),      // 0 — TileIndex::Stone
        gen_dirt(),       // 1 — TileIndex::Dirt
        gen_grass_top(),  // 2 — TileIndex::GrassTop
        gen_grass_side(), // 3 — TileIndex::GrassSide
        gen_sand(),       // 4 — TileIndex::Sand
        gen_water(),      // 5 — TileIndex::Water
        gen_wood(),       // 6 — TileIndex::Wood
        gen_leaves(),     // 7 — TileIndex::Leaves
        gen_bedrock(),    // 8 — TileIndex::Bedrock
    ]
}
