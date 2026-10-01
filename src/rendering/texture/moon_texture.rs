use super::{SKY_TEX_SIZE, blank_rgba, cluster_noise, put_rgba, vary};

/// (centre x, centre y, radius) of each crater, in texture pixels.
const CRATERS: [(i32, i32, i32); 7] = [
    (9, 9, 4),
    (22, 8, 3),
    (16, 18, 5),
    (7, 23, 3),
    (25, 23, 3),
    (26, 15, 2),
    (12, 28, 2),
];

/// Moon — a square pale-blue-grey face with darker craters and a slightly
/// shaded rim. Edit `CRATERS` to move or add craters.
pub fn gen_moon() -> Vec<u8> {
    const S: u64 = 0x300A_1234_ABCD_5678;
    let n = SKY_TEX_SIZE;
    let mut buf = blank_rgba(n);
    for y in 0..n {
        for x in 0..n {
            let edge = x.min(y).min(n - 1 - x).min(n - 1 - y);
            let mut base = if edge < 2 {
                [196, 203, 218] // shaded rim
            } else {
                [226, 231, 243] // surface
            };

            let (xi, yi) = (x as i32, y as i32);
            for &(cx, cy, r) in &CRATERS {
                let d2 = (xi - cx) * (xi - cx) + (yi - cy) * (yi - cy);
                if d2 <= r * r {
                    base = [158, 166, 186]; // crater floor
                } else if d2 <= (r + 1) * (r + 1) && edge >= 2 {
                    base = [238, 242, 252]; // lit crater rim
                }
            }

            put_rgba(&mut buf, n, x, y, vary(base, cluster_noise(x, y, S), 14));
        }
    }
    buf
}

/// Moon glow — a square halo drawn behind the moon: stepped (blocky) rings of
/// pale blue whose alpha fades towards the edge. The middle is hidden by the
/// moon itself.
pub fn gen_moon_glow() -> Vec<u8> {
    let n = SKY_TEX_SIZE;
    let half = n as f32 / 2.0;
    let mut buf = blank_rgba(n);
    for y in 0..n {
        for x in 0..n {
            let dx = (x as f32 + 0.5 - half).abs();
            let dy = (y as f32 + 0.5 - half).abs();
            let d = dx.max(dy) / half; // 0 at centre → 1 at the edge (square rings)
            let band = (d * 8.0) as u32; // 8 stepped rings
            let t = 1.0 - band as f32 / 8.0;
            let alpha = (t * t * 120.0) as u8;
            put_rgba(&mut buf, n, x, y, [205, 220, 255, alpha]);
        }
    }
    buf
}