use super::{TILE_SIZE, blank_tile, cluster_noise, put, sat, variant_salt};

/// Water — deep blue with horizontal sine-wave brightness bands.
/// Alpha is 200 (≈78 %) so underwater geometry shows through
/// (requires the translucent material to actually be used when meshing).
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern.
pub fn gen_water(variant: u64) -> Vec<u8> {
    let s: u64 = 0xBEEF_CAFE_DEAD_BEEF ^ variant_salt(variant);
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let wave = ((y as f32 / 8.0 * std::f32::consts::TAU).sin() * 18.0) as i32;
            let n = cluster_noise(x, y, s) as i32 * 14 / 255 - 7;
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