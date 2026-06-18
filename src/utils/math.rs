/// Convert 3D chunk coordinates to a 1D index.
#[inline]
pub fn chunk_pos_to_idx(cx: i32, cy: i32, cz: i32) -> u64 {
    let cx = cx as u32 as u64;
    let cy = cy as u32 as u64;
    let cz = cz as u32 as u64;
    (cx << 32) | (cy << 16) | cz
}
