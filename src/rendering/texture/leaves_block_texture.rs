use super::{TILE_SIZE, blank_tile, cluster_noise, put, vary, variant_salt};

/// Leaves — two-tone green: dark irregular patches on a lighter base.
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern.
pub fn gen_leaves(variant: u64) -> Vec<u8> {
    let s: u64 = 0xFEED_FACE_C0FF_EEEE ^ variant_salt(variant);
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, s);
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