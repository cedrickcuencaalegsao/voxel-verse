use super::{TILE_SIZE, blank_tile, cluster_noise, put, vary};

/// Leaves — two-tone green: dark irregular patches on a lighter base.
pub fn gen_leaves() -> Vec<u8> {
    const S: u64 = 0xFEED_FACE_C0FF_EEEE;
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let v = cluster_noise(x, y, S);
            let base = if v < 90 {
                [20u8, 110, 20]
            } else {
                [34, 155, 34]
            };
            put(&mut buf, x, y, vary(base, v, 20));
        }
    }
    buf
}