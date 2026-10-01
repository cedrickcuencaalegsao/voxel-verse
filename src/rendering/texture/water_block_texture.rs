use super::{TILE_SIZE, blank_tile, cluster_noise, put, sat};

/// Water — deep blue with horizontal sine-wave brightness bands.
/// Alpha is 200 (≈78 %) so underwater geometry shows through
/// (requires the translucent material to actually be used when meshing).
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