//! Procedural pixel-art block textures — one file per block.
//!
//! Every generator returns a `Vec<u8>` of raw RGBA data for a
//! `TILE_SIZE × TILE_SIZE` tile. No external files or RNG crates are used.
//!
//! Every block generator takes a `variant` number. Variant 0 is the original
//! look; other numbers give a different but stable pattern, so blocks in the
//! world can pick different versions and the texture never visibly repeats.
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

mod moon_texture;
mod star_texture;
mod sun_texture;

pub use bedrock_block_texture::gen_bedrock;
pub use dirt_block_texture::gen_dirt;
pub use grass_side_block_texture::gen_grass_side;
pub use grass_top_block_texture::gen_grass_top;
pub use leaves_block_texture::gen_leaves;
pub use sand_block_texture::gen_sand;
pub use stone_block_texture::gen_stone;
pub use water_block_texture::gen_water;
pub use wood_block_texture::gen_wood;

pub use moon_texture::{gen_moon, gen_moon_glow};
pub use star_texture::{gen_star_dot, gen_star_sparkle};
pub use sun_texture::gen_sun;

pub const TILE_SIZE: u32 = 64;
const CLUSTER: u32 = 4; // pixels per noise cluster edge

// ── Deterministic RNG ─────────────────────────────────────────────────────────

/// Splitmix64 finalizer — bijective, avalanche-quality, branch-free.
#[inline(always)]
pub(crate) fn hash64(mut x: u64) -> u64 {
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

/// XOR-ed into every seed of a generator. Variant 0 returns 0, so the original
/// look is unchanged; other variants give a different, stable salt.
#[inline(always)]
pub(crate) fn variant_salt(variant: u64) -> u64 {
    if variant == 0 { 0 } else { hash64(variant) }
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

// ── Anti-repetition: per-block variation ──────────────────────────────────────

/// How many different versions exist of every tile. Raise it for more variety
/// (the atlas grows by one block of tiles per extra variant).
pub const VARIANTS: usize = 4;

/// Stable pseudo-random value for one face of one block.
/// The same inputs always give the same result, so a block never changes
/// its look between frames or after a chunk is rebuilt.
pub fn block_hash(x: i32, y: i32, z: i32, face: u32) -> u64 {
    let mut h = (x as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    h ^= (y as i64 as u64).wrapping_mul(0x6c62_272e_07bb_0142);
    h ^= (z as i64 as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= (face as u64).wrapping_mul(0x94d0_49bb_1331_11eb);
    hash64(h)
}

// ── Sky textures (sun, moon, stars) ───────────────────────────────────────────
//
// These are not part of the block atlas: `environment/daynight.rs` turns each
// one into its own small image.

/// Edge length (pixels) of the square sun / moon / moon-glow textures.
pub const SKY_TEX_SIZE: u32 = 32;
/// Edge length (pixels) of the star textures.
pub const STAR_TEX_SIZE: u32 = 8;

pub(crate) fn blank_rgba(size: u32) -> Vec<u8> {
    vec![0u8; (size * size * 4) as usize]
}

#[inline(always)]
pub(crate) fn put_rgba(buf: &mut [u8], size: u32, x: u32, y: u32, pixel: [u8; 4]) {
    let i = ((y * size + x) * 4) as usize;
    buf[i..i + 4].copy_from_slice(&pixel);
}

// ── Atlas entry point ─────────────────────────────────────────────────────────

/// Generates one tile in the given variant (0 = original look).
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
/// Any other index (the ore slots) returns a blank transparent tile.
///
/// **This ordering must stay in sync with `TileIndex` in `atlas.rs`.**
pub fn generate_tile(tile: usize, variant: u64) -> Vec<u8> {
    match tile {
        0 => gen_stone(variant),      // TileIndex::Stone
        1 => gen_dirt(variant),       // TileIndex::Dirt
        2 => gen_grass_top(variant),  // TileIndex::GrassTop
        3 => gen_grass_side(variant), // TileIndex::GrassSide
        4 => gen_sand(variant),       // TileIndex::Sand
        5 => gen_water(variant),      // TileIndex::Water
        6 => gen_wood(variant),       // TileIndex::Wood
        7 => gen_leaves(variant),     // TileIndex::Leaves
        8 => gen_bedrock(variant),    // TileIndex::Bedrock
        _ => blank_tile(),
    }
}