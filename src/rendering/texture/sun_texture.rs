use super::{SKY_TEX_SIZE, blank_rgba, cluster_noise, put_rgba, vary};

/// Sun — a square pixel-art sun: hot white-yellow core, then yellow, then an
/// orange rim, with chunky 4px noise so it shimmers like the rest of the
/// block textures. Fully opaque (the sun is a square, not a disc).
pub fn gen_sun() -> Vec<u8> {
    const S: u64 = 0x5011_AAAA_0F0F_1234;
    let n = SKY_TEX_SIZE;
    let mut buf = blank_rgba(n);
    for y in 0..n {
        for x in 0..n {
            // Distance (in pixels) to the nearest edge of the square.
            let edge = x.min(y).min(n - 1 - x).min(n - 1 - y);
            let base = if edge < 2 {
                [255, 160, 40] // orange rim
            } else if edge < 5 {
                [255, 205, 60] // yellow
            } else if edge < 10 {
                [255, 235, 120] // light yellow
            } else {
                [255, 250, 190] // white-hot core
            };
            put_rgba(&mut buf, n, x, y, vary(base, cluster_noise(x, y, S), 24));
        }
    }
    buf
}