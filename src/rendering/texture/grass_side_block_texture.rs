use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary};

/// Grass side — a green cap (~1/6 of height) blending into dirt below.
/// The dirt seed matches `gen_dirt` so the seam looks natural. The cap
/// height jitters per-column and gets a two-row shadowed transition,
/// giving the classic torn/jagged grass-over-dirt silhouette instead of a
/// razor-straight cut.
pub fn gen_grass_side() -> Vec<u8> {
    const S_G: u64 = 0xABCD_1234_EF01_5678;
    const S_D: u64 = 0xCCCC_3333_DDDD_4444; // intentionally matches gen_dirt
    const S_EDGE: u64 = 0x4242_ABAB_1313_C4C4; // per-column cap jitter
    let base_cap = TILE_SIZE / 6; // ≈ 10–11 px green band at the top
    let mut buf = blank_tile();
    for x in 0..TILE_SIZE {
        // Per-column jitter (±2px) on the cap height so the seam reads as
        // jagged grass blades hanging over the dirt, not a straight cut.
        let jitter = (pixel_noise(x, 0, S_EDGE) % 5) as i32 - 2;
        let cap = (base_cap as i32 + jitter).clamp(2, TILE_SIZE as i32 / 3) as u32;
        for y in 0..TILE_SIZE {
            let pixel = if y < cap {
                vary([34, 180, 34], cluster_noise(x, y, S_G), 25)
            } else if y < cap + 2 {
                // Shadowed transition lip right under the jagged edge, so
                // the grass/dirt swap isn't an abrupt hard-edged colour
                // change.
                vary([70, 110, 40], cluster_noise(x, y, S_G), 20)
            } else {
                vary([110, 70, 40], cluster_noise(x, y, S_D), 30)
            };
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}