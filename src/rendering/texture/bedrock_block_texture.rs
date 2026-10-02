use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary, variant_salt};

/// Bedrock — near-black with coarse + fine irregular veins, plus sparse
/// pale mineral speckle.
///
/// `variant` 0 is the original look; other numbers give a different but
/// stable pattern.
pub fn gen_bedrock(variant: u64) -> Vec<u8> {
    let salt = variant_salt(variant);
    let sc: u64 = 0x0000_DEAD_BEEF_0000 ^ salt;
    let sf: u64 = 0x1111_2222_3333_4444 ^ salt;
    let ss: u64 = 0x8888_4444_2222_1111 ^ salt; // sparse speckle
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, sc) / 2 + pixel_noise(x, y, sf) / 2;
            let mut pixel = vary([40, 40, 40], v, 30);
            if pixel_noise(x, y, ss) > 252 {
                pixel = [90, 90, 95, 255];
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}