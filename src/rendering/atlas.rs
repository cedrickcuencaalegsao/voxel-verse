//! Runtime texture atlas: stitches procedural tiles into a single Bevy
//! [`Image`], exposes UV mapping helpers, and registers a [`BlockAtlas`]
//! resource on startup via [`BlockAtlasPlugin`].
//!
//! Atlas layout (256 × 192 px, 4 cols × 3 rows, each tile 64 × 64 px):
//!
//! ```text
//!  col→  0          1        2           3
//! row 0 [Stone    ][Dirt   ][GrassTop  ][GrassSide]
//! row 1 [Sand     ][Water  ][Wood      ][Leaves   ]
//! row 2 [Bedrock  ][CoalOre][IronOre   ][DiamondOre]
//! ```
//!
//! Adding a new block type:
//!   1. Add a generator function in `texture_generator.rs`.
//!   2. Append it to `generate_all_tiles()` — bump `TILE_COUNT`.
//!   3. Add the discriminant to `TileIndex`.
//!   4. Add the match arm to `tile_index_for()`.
//!   (Atlas dimensions recalculate automatically via const arithmetic.)

use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::texture_generator::{TILE_SIZE, generate_all_tiles};
use crate::world::block::BlockKind;

// ── Atlas layout constants ────────────────────────────────────────────────────

/// Total number of distinct tile textures in the atlas.
pub const TILE_COUNT: u32 = 12; // was 9; +3 for CoalOre, IronOre, DiamondOre
/// How many tiles fit across one atlas row.
pub const ATLAS_COLS: u32 = 4;
/// Number of rows required — computed at compile time (ceiling division).
pub const ATLAS_ROWS: u32 = (TILE_COUNT + ATLAS_COLS - 1) / ATLAS_COLS; // = 3
/// Atlas width in pixels.
pub const ATLAS_W: u32 = ATLAS_COLS * TILE_SIZE; // = 256
/// Atlas height in pixels.
pub const ATLAS_H: u32 = ATLAS_ROWS * TILE_SIZE; // = 192

// ── Tile index enum ───────────────────────────────────────────────────────────

/// Canonical tile positions inside the atlas.
///
/// The `#[repr(u32)]` discriminants are the literal flat tile indices used by
/// every UV function in this module.  They MUST match the element order of
/// `generate_all_tiles()` in `texture_generator.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TileIndex {
    Stone = 0,
    Dirt = 1,
    GrassTop = 2,
    GrassSide = 3,
    Sand = 4,
    Water = 5,
    Wood = 6,
    Leaves = 7,
    Bedrock = 8,
    // ── ores (row 2, cols 1-3) ───────────────────────────────────────────────
    // To give each ore a unique texture, add a generator to texture_generator.rs
    // and wire it into generate_all_tiles() at positions 9, 10, 11.
    // Until then they fall back to Stone visually via tile_index_for().
    CoalOre = 9,
    IronOre = 10,
    DiamondOre = 11,
}

// ── Block face enum ───────────────────────────────────────────────────────────

/// The six axis-aligned cube faces.  Used to select per-face textures for
/// blocks like Grass that differ across faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockFace {
    Top,
    Bottom,
    North, // -Z
    South, // +Z
    East,  // +X
    West,  // -X
}

// ── Tile selection ────────────────────────────────────────────────────────────

/// Returns the flat atlas tile index for the given block kind + face.
///
/// This is the single place where "which texture does this block face show?"
/// is decided.  All other UV functions delegate here.
pub fn tile_index_for(kind: BlockKind, face: BlockFace) -> u32 {
    use BlockFace::*;
    use TileIndex::*;

    match kind {
        // Air is never meshed; Stone is a safe visual fallback.
        BlockKind::Air => Stone as u32,

        BlockKind::Stone => Stone as u32,
        BlockKind::Dirt => Dirt as u32,

        // Grass uses three different tile textures depending on face.
        BlockKind::Grass => match face {
            Top => GrassTop as u32,
            Bottom => Dirt as u32, // underside looks like dirt
            _ => GrassSide as u32, // North / South / East / West
        },

        BlockKind::Sand => Sand as u32,
        BlockKind::Water => Water as u32,
        BlockKind::Wood => Wood as u32,
        BlockKind::Leaves => Leaves as u32,
        BlockKind::Bedrock => Bedrock as u32,

        // ── ores ─────────────────────────────────────────────────────────────
        // Each ore has its own TileIndex slot (9-11) so dedicated textures can
        // be added to texture_generator.rs at any time without touching this
        // match.  For now the TileIndex values alias back to Stone in the atlas
        // because generate_all_tiles() only fills 9 entries (indices 0-8).
        // Once you add ore tile generators, flip these to their own TileIndex:
        //   BlockKind::CoalOre    => CoalOre    as u32,
        //   BlockKind::IronOre    => IronOre    as u32,
        //   BlockKind::DiamondOre => DiamondOre as u32,
        BlockKind::CoalOre => Stone as u32, // placeholder → Stone texture
        BlockKind::IronOre => Stone as u32, // placeholder → Stone texture
        BlockKind::DiamondOre => Stone as u32, // placeholder → Stone texture
    }
}

/// Returns `true` when two block faces share the same atlas tile and may be
/// merged by the greedy mesher without UV distortion.
pub fn same_tile(
    kind_a: BlockKind,
    face_a: BlockFace,
    kind_b: BlockKind,
    face_b: BlockFace,
) -> bool {
    tile_index_for(kind_a, face_a) == tile_index_for(kind_b, face_b)
}

// ── UV helpers ────────────────────────────────────────────────────────────────

/// Returns `[u_min, v_min, u_max, v_max]` in normalised 0–1 UV space for the
/// tile at `tile_idx`.
///
/// UV origin is top-left of the atlas image (standard GPU / wgpu convention):
/// `v` increases downward.
pub fn uv_rect(tile_idx: u32) -> [f32; 4] {
    let tw = TILE_SIZE as f32 / ATLAS_W as f32;
    let th = TILE_SIZE as f32 / ATLAS_H as f32;
    let col = tile_idx % ATLAS_COLS;
    let row = tile_idx / ATLAS_COLS;
    let u0 = col as f32 * tw;
    let v0 = row as f32 * th;
    [u0, v0, u0 + tw, v0 + th]
}

/// Returns four UV corners for a single 1×1 block face quad.
///
/// Vertex ordering — **bottom-left, bottom-right, top-right, top-left**
/// (counter-clockwise when viewed from outside the block face).
pub fn quad_uvs_for_tile(tile_idx: u32) -> [[f32; 2]; 4] {
    let [u0, v0, u1, v1] = uv_rect(tile_idx);
    [
        [u0, v1], // 0 — bottom-left
        [u1, v1], // 1 — bottom-right
        [u1, v0], // 2 — top-right
        [u0, v0], // 3 — top-left
    ]
}

/// Convenience wrapper: resolves the tile index for `(kind, face)` and returns
/// the four quad UV corners.  This is what your mesh builder calls per face.
pub fn quad_uvs_for(kind: BlockKind, face: BlockFace) -> [[f32; 2]; 4] {
    quad_uvs_for_tile(tile_index_for(kind, face))
}

// ── Atlas image builder ───────────────────────────────────────────────────────

/// Stitches all procedural tile buffers into a single RGBA [`Image`] with
/// nearest-neighbour filtering.
pub fn build_atlas_image() -> Image {
    let tiles = generate_all_tiles();
    let mut rgba = vec![0u8; (ATLAS_W * ATLAS_H * 4) as usize];

    for (idx, tile) in tiles.iter().enumerate() {
        let col = (idx as u32) % ATLAS_COLS;
        let row = (idx as u32) / ATLAS_COLS;
        let ox = col * TILE_SIZE;
        let oy = row * TILE_SIZE;

        for ty in 0..TILE_SIZE {
            for tx in 0..TILE_SIZE {
                let src = ((ty * TILE_SIZE + tx) * 4) as usize;
                let dst = (((oy + ty) * ATLAS_W + (ox + tx)) * 4) as usize;
                rgba[dst..dst + 4].copy_from_slice(&tile[src..src + 4]);
            }
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: ATLAS_W,
            height: ATLAS_H,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}

// ── Bevy resource ─────────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct BlockAtlas {
    /// Material for stone, dirt, grass, sand, wood, leaves, bedrock, ores.
    pub opaque: Handle<StandardMaterial>,
    /// Material for water (semi-transparent, alpha blended).
    pub translucent: Handle<StandardMaterial>,
}

// ── Startup system ────────────────────────────────────────────────────────────

pub fn setup_block_atlas(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tex = images.add(build_atlas_image());

    let opaque = materials.add(StandardMaterial {
        base_color_texture: Some(tex.clone()),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        alpha_mode: AlphaMode::Opaque,
        ..Default::default()
    });

    let translucent = materials.add(StandardMaterial {
        base_color_texture: Some(tex),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    commands.insert_resource(BlockAtlas {
        opaque,
        translucent,
    });
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct BlockAtlasPlugin;

impl Plugin for BlockAtlasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_block_atlas);
    }
}
