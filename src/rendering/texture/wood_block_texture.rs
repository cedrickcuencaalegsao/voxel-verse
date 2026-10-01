use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, sat, vary};

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