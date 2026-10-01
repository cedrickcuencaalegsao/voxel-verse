use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, sat, vary};

/// Grass top — bright green with cluster variation, per-column blade
/// streaks, and coarse moss/shadow patches for a less flat, less
/// obviously-tiled look.
pub fn gen_grass_top() -> Vec<u8> {
    const S: u64 = 0xEEEE_5555_FFFF_6666;
    const S_BLADE: u64 = 0x2727_9595_4646_ABAB; // per-column blade tone
    const S_PATCH: u64 = 0x6262_1414_D2D2_0808; // coarse moss/shadow patches
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut pixel = vary([34, 180, 34], cluster_noise(x, y, S), 30);

            // Thin vertical blade streaks: shift green per-column (not
            // per-pixel) so the top reads as individual grass blades
            // rather than a uniform wash.
            let blade_shift = pixel_noise(x, 0, S_BLADE) as i32 / 32 - 4; // -4..+3
            pixel[1] = sat(pixel[1] as i32 + blade_shift);

            // Occasional darker moss/shadow patches, coarser than the base
            // cluster noise (double-wide clusters), to break up repetition
            // at a glance rather than every 4px.
            if cluster_noise(x / 2, y / 2, S_PATCH) > 235 {
                pixel = vary([22, 130, 22], pixel_noise(x, y, S), 12);
            }

            put(&mut buf, x, y, pixel);
        }
    }
    buf
}