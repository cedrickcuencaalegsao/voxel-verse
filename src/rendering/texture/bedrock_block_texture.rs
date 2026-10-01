use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary};

/// Bedrock — near-black with coarse + fine irregular veins, plus sparse
/// pale mineral speckle.
pub fn gen_bedrock() -> Vec<u8> {
    const SC: u64 = 0x0000_DEAD_BEEF_0000;
    const SF: u64 = 0x1111_2222_3333_4444;
    const SS: u64 = 0x8888_4444_2222_1111; // sparse speckle
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, SC) / 2 + pixel_noise(x, y, SF) / 2;
            let mut pixel = vary([40, 40, 40], v, 30);
            if pixel_noise(x, y, SS) > 252 {
                pixel = [90, 90, 95, 255];
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}