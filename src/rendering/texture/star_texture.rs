use super::{STAR_TEX_SIZE, blank_rgba, put_rgba};

/// Turns an 8x8 character mask into RGBA pixels.
///   'W' = white core, 'b' = bright, 'a' = faint glow, anything else = transparent.
fn from_mask(mask: [&str; 8]) -> Vec<u8> {
    let n = STAR_TEX_SIZE;
    let mut buf = blank_rgba(n);
    for (y, row) in mask.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let pixel = match c {
                'W' => [255, 255, 255, 255],
                'b' => [215, 225, 255, 210],
                'a' => [190, 205, 255, 110],
                _ => continue,
            };
            put_rgba(&mut buf, n, x as u32, y as u32, pixel);
        }
    }
    buf
}

/// Star (dot) — a small bright square with a faint halo.
pub fn gen_star_dot() -> Vec<u8> {
    from_mask([
        "........",
        "........",
        "..aaaa..",
        "..aWWa..",
        "..aWWa..",
        "..aaaa..",
        "........",
        "........",
    ])
}

/// Star (sparkle) — a plus-shaped twinkling star.
pub fn gen_star_sparkle() -> Vec<u8> {
    from_mask([
        "...aa...",
        "...aa...",
        "...bb...",
        "aabWWbaa",
        "aabWWbaa",
        "...bb...",
        "...aa...",
        "...aa...",
    ])
}