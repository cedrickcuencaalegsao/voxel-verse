use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary};

/// Dirt — warm brown with slightly darker cluster patches plus sparse
/// pebble/root flecks.
///
/// NOTE: `grass_side_block_texture.rs` reuses this seed so the dirt part of
/// the grass side matches. Change it in both places if you change it here.
pub fn gen_dirt() -> Vec<u8> {
    const S: u64 = 0xCCCC_3333_DDDD_4444;
    const S_PEBBLE: u64 = 0x7777_1212_8888_3434;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut pixel = vary([110, 70, 40], cluster_noise(x, y, S), 35);
            // A sparse single-pixel darker fleck layer (small pebbles /
            // root bits) so dirt doesn't read as a flat blocky wash.
            if pixel_noise(x, y, S_PEBBLE) > 250 {
                pixel = [80, 55, 30, 255];
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}