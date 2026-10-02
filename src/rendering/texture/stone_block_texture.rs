use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary, variant_salt};

/// Stone — uniform grey with blocky noise patches plus sparse mineral flecks.
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern.
pub fn gen_stone(variant: u64) -> Vec<u8> {
    let salt = variant_salt(variant);
    let s: u64 = 0xAAAA_1111_BBBB_2222 ^ salt;
    let s_fleck: u64 = 0x1919_8181_2020_9090 ^ salt; // sparse pixel-level flecks
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut pixel = vary([120, 120, 120], cluster_noise(x, y, s), 40);
            // A sparse single-pixel fleck layer on top of the blocky base
            // noise so stone reads as speckled rock instead of flat grey
            // squares (~4% of pixels combined).
            let fleck = pixel_noise(x, y, s_fleck);
            if fleck > 245 {
                pixel = [200, 200, 205, 255]; // bright quartz-like fleck
            } else if fleck < 10 {
                pixel = [70, 70, 75, 255]; // dark mineral fleck
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}