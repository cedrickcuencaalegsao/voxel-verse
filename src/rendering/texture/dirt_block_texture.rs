use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary, variant_salt};

/// Dirt — warm brown with slightly darker cluster patches plus sparse
/// pebble/root flecks.
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern (used to avoid visible repetition).
///
/// NOTE: `grass_side_block_texture.rs` reuses the base seed so the dirt part of
/// the grass side matches at variant 0.
pub fn gen_dirt(variant: u64) -> Vec<u8> {
    let salt = variant_salt(variant);
    let s: u64 = 0xCCCC_3333_DDDD_4444 ^ salt;
    let s_pebble: u64 = 0x7777_1212_8888_3434 ^ salt;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut pixel = vary([110, 70, 40], cluster_noise(x, y, s), 35);
            // A sparse single-pixel darker fleck layer (small pebbles /
            // root bits) so dirt doesn't read as a flat blocky wash.
            if pixel_noise(x, y, s_pebble) > 250 {
                pixel = [80, 55, 30, 255];
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}