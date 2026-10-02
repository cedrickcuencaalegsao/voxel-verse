use super::{TILE_SIZE, blank_tile, cluster_noise, hash64, pixel_noise, put, vary};

/// Grass side — a green cap (~1/6 of height) blending into dirt below.
/// The cap height jitters per-column and gets a two-row shadowed transition,
/// giving the classic torn/jagged grass-over-dirt silhouette.
///
/// `variant` 0 is the original look (identical to before). Any other number
/// gives a different but still stable pattern, used to avoid repetition.
pub fn gen_grass_side(variant: u64) -> Vec<u8> {
    let salt = if variant == 0 { 0 } else { hash64(variant) };
    let s_g: u64 = 0xABCD_1234_EF01_5678 ^ salt;
    let s_d: u64 = 0xCCCC_3333_DDDD_4444 ^ salt;
    let s_edge: u64 = 0x4242_ABAB_1313_C4C4 ^ salt; // per-column cap jitter
    let base_cap = TILE_SIZE / 6; // ≈ 10–11 px green band at the top
    let mut buf = blank_tile();
    for x in 0..TILE_SIZE {
        // Per-column jitter (±2px) on the cap height so the seam reads as
        // jagged grass blades hanging over the dirt, not a straight cut.
        let jitter = (pixel_noise(x, 0, s_edge) % 5) as i32 - 2;
        let cap = (base_cap as i32 + jitter).clamp(2, TILE_SIZE as i32 / 3) as u32;
        for y in 0..TILE_SIZE {
            let pixel = if y < cap {
                vary([34, 180, 34], cluster_noise(x, y, s_g), 25)
            } else if y < cap + 2 {
                // Shadowed transition lip right under the jagged edge.
                vary([70, 110, 40], cluster_noise(x, y, s_g), 20)
            } else {
                vary([110, 70, 40], cluster_noise(x, y, s_d), 30)
            };
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}