use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary, variant_salt};

/// Sand — warm beige with layered coarse + fine grain, plus sparse darker
/// grain specks.
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern.
pub fn gen_sand(variant: u64) -> Vec<u8> {
    let salt = variant_salt(variant);
    let sc: u64 = 0x1122_AABB_3344_CCDD ^ salt;
    let sf: u64 = 0x5566_EEFF_7788_0011 ^ salt;
    let ss: u64 = 0x9911_2233_4455_6677 ^ salt; // sparse dark grain specks
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, sc) / 2 + pixel_noise(x, y, sf) / 2;
            let mut pixel = vary([220, 210, 120], v, 25);
            if pixel_noise(x, y, ss) > 250 {
                pixel = [180, 165, 90, 255];
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}