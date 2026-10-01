//! Procedural pixel-art block textures — one file per block.
//!
//! Every generator returns a `Vec<u8>` of raw RGBA data for a
//! `TILE_SIZE × TILE_SIZE` tile. No external files or RNG crates are used.
//!
//! To edit a block's default look, open its `<block>_block_texture.rs` file.
//! Shared noise / colour helpers live here.

mod bedrock_block_texture;
mod dirt_block_texture;
mod grass_side_block_texture;
mod grass_top_block_texture;
mod leaves_block_texture;
mod sand_block_texture;
mod stone_block_texture;
mod water_block_texture;
mod wood_block_texture;

pub use bedrock_block_texture::gen_bedrock;
pub use dirt_block_texture::gen_dirt;
pub use grass_side_block_texture::gen_grass_side;
pub use grass_top_block_texture::gen_grass_top;
pub use leaves_block_texture::gen_leaves;
pub use sand_block_texture::gen_sand;
pub use stone_block_texture::gen_stone;
pub use water_block_texture::gen_water;
pub use wood_block_texture::gen_wood;

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
pub(crate) fn cluster_noise(px: u32, py: u32, seed: u64) -> u8 {
    let x = ((px / CLUSTER) as u64).wrapping_mul(0x9e3779b97f4a7c15);
    let y = ((py / CLUSTER) as u64).wrapping_mul(0x6c62272e07bb0142);

    let h = hash64(seed ^ x ^ y);
    (h >> 56) as u8
}

#[inline(always)]
pub(crate) fn pixel_noise(px: u32, py: u32, seed: u64) -> u8 {
    let x = (px as u64).wrapping_mul(0x9e3779b97f4a7c15);
    let y = (py as u64).wrapping_mul(0x6c62272e07bb0142);

    let h = hash64(seed ^ x ^ y);
    (h >> 56) as u8
}

// ── Colour helpers ────────────────────────────────────────────────────────────

#[inline(always)]
pub(crate) fn sat(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

/// Shift every channel of `base` by the same signed delta derived from
/// `noise_byte`. `range` controls the maximum deviation (±range/2).
/// Returns a full RGBA pixel with alpha = 255.
#[inline(always)]
pub(crate) fn vary(base: [u8; 3], noise_byte: u8, range: i32) -> [u8; 4] {
    let d = noise_byte as i32 * range / 255 - range / 2;
    [
        sat(base[0] as i32 + d),
        sat(base[1] as i32 + d),
        sat(base[2] as i32 + d),
        255,
    ]
}

// ── Tile utilities ────────────────────────────────────────────────────────────

pub(crate) fn blank_tile() -> Vec<u8> {
    vec![0u8; (TILE_SIZE * TILE_SIZE * 4) as usize]
}

#[inline(always)]
pub(crate) fn put(buf: &mut [u8], x: u32, y: u32, pixel: [u8; 4]) {
    let i = ((y * TILE_SIZE + x) * 4) as usize;
    buf[i..i + 4].copy_from_slice(&pixel);
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