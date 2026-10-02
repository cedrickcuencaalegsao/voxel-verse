use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::texture::{TILE_SIZE, VARIANTS, generate_tile};
use crate::world::block::BlockKind;

pub const TILE_COUNT: u32 = 12; // was 9; +3 for CoalOre, IronOre, DiamondOre

/// Atlas slots in total: every tile has `VARIANTS` different versions.
///
/// Layout: slots `0..TILE_COUNT` hold variant 0 (the originals, so all the old
/// tile indices still work), slots `TILE_COUNT..2*TILE_COUNT` hold variant 1,
/// and so on.
pub const ATLAS_SLOTS: u32 = TILE_COUNT * VARIANTS as u32;
/// How many tiles fit across one atlas row.
pub const ATLAS_COLS: u32 = 4;
/// Number of rows required — computed at compile time (ceiling division).
pub const ATLAS_ROWS: u32 = (ATLAS_SLOTS + ATLAS_COLS - 1) / ATLAS_COLS;
/// Atlas width in pixels.
pub const ATLAS_W: u32 = ATLAS_COLS * TILE_SIZE;
/// Atlas height in pixels.
pub const ATLAS_H: u32 = ATLAS_ROWS * TILE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[allow(dead_code)]
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
    CoalOre = 9,
    IronOre = 10,
    DiamondOre = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockFace {
    Top,
    Bottom,
    North,
    South,
    East,
    West,
}

pub fn tile_index_for(kind: BlockKind, face: BlockFace) -> u32 {
    use BlockFace::*;
    use TileIndex::*;

    match kind {
        BlockKind::Air => Stone as u32,

        BlockKind::Stone => Stone as u32,
        BlockKind::Dirt => Dirt as u32,
        BlockKind::Grass => match face {
            Top => GrassTop as u32,
            Bottom => Dirt as u32,
            _ => GrassSide as u32,
        },

        BlockKind::Sand => Sand as u32,
        BlockKind::Water => Water as u32,
        BlockKind::Wood => Wood as u32,
        BlockKind::Leaves => Leaves as u32,
        BlockKind::Bedrock => Bedrock as u32,

        BlockKind::CoalOre => Stone as u32,
        BlockKind::IronOre => Stone as u32,
        BlockKind::DiamondOre => Stone as u32,
    }
}

#[allow(dead_code)]
pub fn same_tile(
    kind_a: BlockKind,
    face_a: BlockFace,
    kind_b: BlockKind,
    face_b: BlockFace,
) -> bool {
    tile_index_for(kind_a, face_a) == tile_index_for(kind_b, face_b)
}

pub fn uv_rect(tile_idx: u32) -> [f32; 4] {
    let tw = TILE_SIZE as f32 / ATLAS_W as f32;
    let th = TILE_SIZE as f32 / ATLAS_H as f32;
    let col = tile_idx % ATLAS_COLS;
    let row = tile_idx / ATLAS_COLS;
    let u0 = col as f32 * tw;
    let v0 = row as f32 * th;
    // Inset by half a texel so sampling never touches the neighbouring tile.
    let eu = 0.5 / ATLAS_W as f32;
    let ev = 0.5 / ATLAS_H as f32;
    [u0 + eu, v0 + ev, u0 + tw - eu, v0 + th - ev]
}

pub fn quad_uvs_for_tile(tile_idx: u32) -> [[f32; 2]; 4] {
    let [u0, v0, u1, v1] = uv_rect(tile_idx);
    [
        [u0, v1], // 0 — bottom-left
        [u1, v1], // 1 — bottom-right
        [u1, v0], // 2 — top-right
        [u0, v0], // 3 — top-left
    ]
}

/// Plain (variant 0) UVs. Existing mesher code that calls this keeps working.
#[allow(dead_code)]
pub fn quad_uvs_for(kind: BlockKind, face: BlockFace) -> [[f32; 2]; 4] {
    quad_uvs_for_tile(tile_index_for(kind, face))
}

// ── Anti-repetition ───────────────────────────────────────────────────────────

/// Atlas slot of `tile_idx` in the given variant (0..VARIANTS).
pub fn variant_slot(tile_idx: u32, variant: usize) -> u32 {
    variant as u32 * TILE_COUNT + tile_idx
}

/// Picks a variant for one block face from its `block_hash`.
/// The variants are separately generated patterns (not flipped copies), so
/// every face, side or top, can safely use any of them.
pub fn pick_variant(hash: u64) -> usize {
    ((hash >> 4) % VARIANTS as u64) as usize
}

/// UVs for one block face with its own random-but-stable variant.
/// `hash` comes from `texture::block_hash(wx, wy, wz, face_id)`.
pub fn quad_uvs_for_hashed(kind: BlockKind, face: BlockFace, hash: u64) -> [[f32; 2]; 4] {
    let tile = tile_index_for(kind, face);
    quad_uvs_for_tile(variant_slot(tile, pick_variant(hash)))
}

/// Small per-face brightness change (about 0.92 to 1.04) for the vertex color,
/// so neighbouring blocks never look exactly the same.
pub fn face_tint(_hash: u64) -> f32 {
    // 0.92 + ((_hash >> 16) & 0xFF) as f32 / 255.0 * 0.12 // visible like of each voxel
    1.0
}

// ── Atlas image ───────────────────────────────────────────────────────────────

pub fn build_atlas_image() -> Image {
    let mut rgba = vec![0u8; (ATLAS_W * ATLAS_H * 4) as usize];

    for variant in 0..VARIANTS {
        for tile_idx in 0..TILE_COUNT {
            let tile = generate_tile(tile_idx as usize, variant as u64);
            let slot = variant_slot(tile_idx, variant);
            let ox = (slot % ATLAS_COLS) * TILE_SIZE;
            let oy = (slot / ATLAS_COLS) * TILE_SIZE;

            for ty in 0..TILE_SIZE {
                for tx in 0..TILE_SIZE {
                    let src = ((ty * TILE_SIZE + tx) * 4) as usize;
                    let dst = (((oy + ty) * ATLAS_W + (ox + tx)) * 4) as usize;
                    rgba[dst..dst + 4].copy_from_slice(&tile[src..src + 4]);
                }
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

#[derive(Resource)]
pub struct BlockAtlas {
    pub opaque: Handle<StandardMaterial>,
    #[allow(dead_code)]
    pub translucent: Handle<StandardMaterial>,
}

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

pub struct BlockAtlasPlugin;

impl Plugin for BlockAtlasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_block_atlas);
    }
}