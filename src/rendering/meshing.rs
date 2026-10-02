use crate::rendering::atlas::{face_tint, quad_uvs_for_hashed};
use crate::rendering::greedy_meshing::generate_chunk_quads;
use crate::rendering::materials::{BlockAtlas, BlockFace};
use crate::rendering::texture::block_hash;
use crate::world::chunk::Chunk;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

/// Tangent axes (u, v) for a face such that u.cross(v) == that face's
/// outward normal. This must be picked per direction, not shared between
/// +X/-X, +Y/-Y, +Z/-Z — reusing the same pair for both signs flips the
/// triangle winding for one of the two, which is why half of every block's
/// faces were getting back-face-culled.
fn face_tangents(normal: [f32; 3]) -> (Vec3, Vec3) {
    match normal {
        // Side faces: v is always +Y so the grass cap stays on top.
        // u is chosen so u.cross(v) == outward normal (keeps winding correct).
        [1.0, 0.0, 0.0] => (Vec3::NEG_Z, Vec3::Y), // +X
        [-1.0, 0.0, 0.0] => (Vec3::Z, Vec3::Y),    // -X
        [0.0, 0.0, 1.0] => (Vec3::X, Vec3::Y),     // +Z
        [0.0, 0.0, -1.0] => (Vec3::NEG_X, Vec3::Y), // -Z

        // Top / bottom: orientation doesn't matter visually.
        [0.0, 1.0, 0.0] => (Vec3::Z, Vec3::X),  // +Y
        [0.0, -1.0, 0.0] => (Vec3::X, Vec3::Z), // -Y

        _ => (Vec3::X, Vec3::Y),
    }
}

/// Maps a quad's world-space normal to the logical [`BlockFace`] used to
/// pick the correct atlas tile (e.g. Grass top vs. side vs. bottom).
fn block_face_from_normal(normal: [f32; 3]) -> BlockFace {
    match normal {
        [0.0, 1.0, 0.0] => BlockFace::Top,
        [0.0, -1.0, 0.0] => BlockFace::Bottom,
        [0.0, 0.0, -1.0] => BlockFace::North,
        [0.0, 0.0, 1.0] => BlockFace::South,
        [1.0, 0.0, 0.0] => BlockFace::East,
        [-1.0, 0.0, 0.0] => BlockFace::West,
        _ => BlockFace::Top, // unreachable for axis-aligned voxel meshes
    }
}

pub fn remesh_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    atlas: Res<BlockAtlas>,
    query: Query<(Entity, &Chunk, Option<&Transform>), Changed<Chunk>>,
) {
    for (entity, chunk, transform) in query.iter() {
        if !chunk.needs_remesh {
            continue;
        }

        // The mesh is drawn with the chunk entity's transform, so a block's
        // WORLD position is that translation plus the quad's local position.
        // Hashing the world position (not the chunk-local one) is what stops
        // every chunk from repeating the same pattern.
        let chunk_origin = transform.map(|t| t.translation).unwrap_or(Vec3::ZERO);

        let quads = generate_chunk_quads(chunk);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_index = 0u32;

        for quad in &quads {
            // quad.position is the block's MIN corner — block (3,5,2) occupies
            // [3,4]x[5,6]x[2,3] — so we move to the block's center first, then
            // push out half a unit along the normal to reach the face plane.
            let block_center = Vec3::from_array(quad.position) + Vec3::splat(0.5);
            let normal = Vec3::from_array(quad.normal);
            let face_center = block_center + normal * 0.5;

            let (u_dir, v_dir) = face_tangents(quad.normal);
            let w = quad.size[0];
            let h = quad.size[1];

            let v0 = face_center - u_dir * (w * 0.5) - v_dir * (h * 0.5);
            let v1 = v0 + u_dir * w;
            let v2 = v0 + u_dir * w + v_dir * h;
            let v3 = v0 + v_dir * h;

            positions.extend_from_slice(&[
                v0.to_array(),
                v1.to_array(),
                v2.to_array(),
                v3.to_array(),
            ]);
            normals.extend_from_slice(&[quad.normal; 4]);

            // Per-block variation: a stable hash of the block's world position
            // and face picks which version of the tile to use, plus a small
            // brightness tint, so the pattern never visibly repeats.
            let face = block_face_from_normal(quad.normal);
            let world_min = chunk_origin + Vec3::from_array(quad.position);
            let hash = block_hash(
                world_min.x.floor() as i32,
                world_min.y.floor() as i32,
                world_min.z.floor() as i32,
                face as u32,
            );
            uvs.extend_from_slice(&quad_uvs_for_hashed(quad.block_kind, face, hash));

            let tint = face_tint(hash);
            colors.extend_from_slice(&[[tint, tint, tint, 1.0]; 4]);

            indices.extend_from_slice(&[
                vertex_index,
                vertex_index + 1,
                vertex_index + 2,
                vertex_index + 2,
                vertex_index + 3,
                vertex_index,
            ]);
            vertex_index += 4;
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U32(indices));

        // NOTE: all quads in a chunk currently share one opaque, textured
        // material. Water blocks will render using the atlas texture's
        // baked-in alpha=200 pixels, but AlphaMode::Opaque ignores alpha,
        // so water will currently look fully solid instead of translucent.
        //
        // To get real water transparency, split `quads` into two groups
        // (water vs. everything else) before the loop above, build two
        // meshes, and spawn two entities — one with `atlas.opaque`, one
        // with `atlas.translucent`.
        commands.entity(entity).insert((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(atlas.opaque.clone()),
        ));
    }
}