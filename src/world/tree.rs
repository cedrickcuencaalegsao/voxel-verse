use super::block::BlockKind;

pub type TreeBlocks = Vec<(i32, i32, i32, BlockKind)>;

#[allow(dead_code)]
pub fn oak_tree_blocks(trunk_height: i32) -> TreeBlocks {
    let mut blocks = Vec::new();

    for y in 0..trunk_height {
        blocks.push((0, y, 0, BlockKind::Wood));
    }

    let canopy_center_y = trunk_height + 1;
    let radius = 2i32;

    for dy in -1..=2 {
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let dist_sq = dx * dx + dz * dz + (dy * dy) / 2;
                if dist_sq > radius * radius + 1 {
                    continue;
                }
                if dx == 0 && dz == 0 && dy < 0 {
                    continue;
                }
                blocks.push((dx, canopy_center_y + dy, dz, BlockKind::Leaves));
            }
        }
    }
    blocks
}

#[allow(dead_code)]
pub fn pine_tree_blocks(trunk_height: i32) -> TreeBlocks {
    let mut blocks = Vec::new();

    for y in 0..trunk_height {
        blocks.push((0, y, 0, BlockKind::Wood));

        if y < 2 {
            blocks.push((1, y, 0, BlockKind::Wood));
            blocks.push((0, y, 1, BlockKind::Wood));
            blocks.push((1, y, 1, BlockKind::Wood));
        } else if y < trunk_height / 2 {
            if y % 2 == 0 {
                blocks.push((1, y, 0, BlockKind::Wood));
            } else {
                blocks.push((0, y, 1, BlockKind::Wood));
            }
        }
    }

    let canopy_start = trunk_height - 2;
    let canopy_layers = trunk_height + 4;

    for layer in 0..canopy_layers {
        let y = canopy_start + layer;
        let max_radius = 5;
        let t = layer as f32 / canopy_layers as f32;
        let radius = ((1.0 - t) * max_radius as f32).round() as i32;

        if radius == 0 {
            push_leaf(&mut blocks, 0, y, 0);
            continue;
        }

        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let dist_sq = dx * dx + dz * dz;
                let r_sq = radius * radius;
                let is_ring_layer = layer > canopy_layers / 2;
                let inner_r = (radius - 1).max(0);

                if dist_sq <= r_sq {
                    if !is_ring_layer || dist_sq >= inner_r * inner_r {
                        push_leaf(&mut blocks, dx, y, dz);
                    }
                }
            }
        }

        if layer % 3 == 0 && radius >= 2 {
            add_diagonal_branches(&mut blocks, y, radius);
        }
    }

    blocks
}

fn add_diagonal_branches(blocks: &mut TreeBlocks, y: i32, radius: i32) {
    let directions = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];

    for (dx, dz) in directions {
        let branch_len = (radius - 1).max(1);

        for step in 1..=branch_len {
            let bx = dx * step;
            let bz = dz * step;
            blocks.push((bx, y, bz, BlockKind::Wood));

            if step == branch_len {
                blocks.push((bx, y - 1, bz, BlockKind::Wood));
                push_leaf(blocks, bx, y - 1, bz + 1);
                push_leaf(blocks, bx, y - 1, bz - 1);
                push_leaf(blocks, bx + 1, y - 1, bz);
                push_leaf(blocks, bx - 1, y - 1, bz);
                push_leaf(blocks, bx, y, bz);
            }
        }

        blocks.push((dx, y, dz, BlockKind::Wood));
    }
}

#[inline]
fn push_leaf(blocks: &mut TreeBlocks, x: i32, y: i32, z: i32) {
    blocks.push((x, y, z, BlockKind::Leaves));
}
