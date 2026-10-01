use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary};

/// Sand — warm beige with layered coarse + fine grain, plus sparse darker
/// grain specks.
pub fn gen_sand() -> Vec<u8> {
    const SC: u64 = 0x1122_AABB_3344_CCDD;
    const SF: u64 = 0x5566_EEFF_7788_0011;
    const SS: u64 = 0x9911_2233_4455_6677; // sparse dark grain specks
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, SC) / 2 + pixel_noise(x, y, SF) / 2;
            let mut pixel = vary([220, 210, 120], v, 25);
            if pixel_noise(x, y, SS) > 250 {
                pixel = [180, 165, 90, 255];
            }
            put(&mut buf, x, y, pixel);
        }
    }
    buf
}