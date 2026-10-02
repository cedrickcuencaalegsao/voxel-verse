use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, sat, vary, variant_salt};

/// Wood (log side) — warm brown with wavy vertical grain lines.
/// The grain wobble is driven by a separate per-row seed so lines aren't
/// perfectly straight, matching Minecraft's log appearance.
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern.
pub fn gen_wood(variant: u64) -> Vec<u8> {
    let salt = variant_salt(variant);
    let sg: u64 = 0x1357_2468_9ABC_DEF0 ^ salt; // grain wobble per row
    let sb: u64 = 0xAABB_CCDD_EEFF_0011 ^ salt; // base colour cluster noise
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            // Shift x slightly per row to create a wavy grain effect.
            let wobble = pixel_noise(0, y, sg) as i32 / 16 - 8;
            let grain_x = (x as i32 + wobble).rem_euclid(TILE_SIZE as i32) as u32;
            let on_grain = (grain_x % 8) < 2;
            let dark = if on_grain { 28i32 } else { 0 };
            let base = [sat(120 - dark), sat(80 - dark), sat(40 - dark)];
            put(&mut buf, x, y, vary(base, cluster_noise(x, y, sb), 18));
        }
    }
    buf
}