use super::{TILE_SIZE, blank_tile, cluster_noise, pixel_noise, put, vary};

/// Stone — uniform grey with blocky noise patches plus sparse mineral flecks.
pub fn gen_stone() -> Vec<u8> {
    const S: u64 = 0xAAAA_1111_BBBB_2222;
    const S_FLECK: u64 = 0x1919_8181_2020_9090; // sparse pixel-level flecks
    let mut buf = blank_tile();
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut pixel = vary([120, 120, 120], cluster_noise(x, y, S), 40);
            // A sparse single-pixel fleck layer on top of the blocky base
            // noise so stone reads as speckled rock instead of flat grey
            // squares. Thresholds are chosen so flecks stay rare (~4% of
            // pixels combined).
            let fleck = pixel_noise(x, y, S_FLECK);
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