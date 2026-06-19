// use crate::world::block::BlockKind;
// use bevy::prelude::*;

// /// Flat placeholder color per block kind, used for chunk-mesh vertex
// /// coloring until a real texture atlas is wired up.
// pub fn color_for_kind(kind: BlockKind) -> Color {
//     match kind {
//         BlockKind::Air => Color::srgba(0.0, 0.0, 0.0, 0.0),
//         BlockKind::Stone => Color::srgb(0.5, 0.5, 0.5),
//         BlockKind::Dirt => Color::srgb(0.6, 0.4, 0.2),
//         BlockKind::Grass => Color::srgb(0.2, 0.8, 0.2),
//         BlockKind::Sand => Color::srgb(0.9, 0.8, 0.5),
//         BlockKind::Water => Color::srgba(0.1, 0.2, 0.9, 0.5),
//         BlockKind::Wood => Color::srgb(0.4, 0.2, 0.0),
//         BlockKind::Leaves => Color::srgb(0.1, 0.8, 0.1),
//         BlockKind::Bedrock => Color::srgb(0.1, 0.1, 0.1),
//     }
// }

//! Block material utilities.
//!
//! ## What changed
//! `color_for_kind` has been **removed**.  Block appearance is now driven
//! entirely by the procedural texture atlas; see [`super::atlas`] for the
//! authoritative API.
//!
//! This file re-exports the atlas symbols so existing import paths in
//! `greedy_meshing.rs` and elsewhere require the minimum number of changes:
//!
//! ```rust,ignore
//! // Before:
//! use crate::rendering::material::color_for_kind;
//!
//! // After (only the symbol names change, the use-path prefix stays the same):
//! use crate::rendering::material::{quad_uvs_for, BlockFace};
//! ```

pub use super::atlas::{
    // ── Constants ──────────────────────────────────────────────────────────
    ATLAS_COLS,
    ATLAS_H,
    ATLAS_W,
    // ── Types ──────────────────────────────────────────────────────────────
    BlockAtlas,
    BlockFace,
    TILE_COUNT,

    TileIndex,

    // Four CCW quad UV corners resolved from a `(BlockKind, BlockFace)` pair.
    // **This is the primary function to call from `greedy_meshing.rs`.**
    quad_uvs_for,
    // Four CCW quad UV corners `[[f32;2];4]` by tile index.
    quad_uvs_for_tile,
    // True when two (kind, face) combinations share the same tile — used as
    // the greedy-merge predicate.
    same_tile,
    // Raw tile index for a (kind, face) pair.
    tile_index_for,
    // ── UV helpers — these replace color_for_kind in the mesh builder ──────
    // `[u_min, v_min, u_max, v_max]` for a tile index.
    uv_rect,
};
