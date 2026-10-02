use super::{TILE_SIZE, blank_tile, cluster_noise, hash64, pixel_noise, put, sat, vary};

/// Grass top — bright green with cluster variation, per-column blade
/// streaks, and coarse moss/shadow patches.
///
/// `variant` 0 is the original look (identical to before). Any other number
/// gives a different but still stable pattern, used to avoid repetition.
pub fn gen_grass_top(variant: u64) -> Vec<u8> {
    let salt = if variant == 0 { 0 } else { hash64(variant) };
    let s: u64 = 0xEEEE_5555_FFFF_6666 ^ salt;
    let s_blade: u64 = 0x2727_9595_4646_ABAB ^ salt; // per-column blade tone
    let s_patch: u64 = 0x6262_1414_D2D2_0808 ^ salt; // coarse moss/shadow patches
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut pixel = vary([34, 180, 34], cluster_noise(x, y, s), 30);

            // Thin vertical blade streaks: shift green per-column.
            let blade_shift = pixel_noise(x, 0, s_blade) as i32 / 32 - 4; // -4..+3
            pixel[1] = sat(pixel[1] as i32 + blade_shift);

            // Occasional darker moss/shadow patches, coarser than the base
            // cluster noise.
            if cluster_noise(x / 2, y / 2, s_patch) > 235 {
                pixel = vary([22, 130, 22], pixel_noise(x, y, s), 12);
            }

            put(&mut buf, x, y, pixel);
        }
    }
    buf
}