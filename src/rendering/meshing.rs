use crate::rendering::atlas::{face_tint, quad_uvs_for_hashed};
use crate::rendering::greedy_meshing::generate_chunk_quads;
use crate::rendering::materials::{BlockAtlas, BlockFace};
use crate::rendering::texture::block_hash;
use crate::world::block::BlockKind;
use crate::world::chunk::Chunk;
use bevy::asset::RenderAssetUsages;
use bevy::light::NotShadowCaster;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

/// Points from a chunk entity to its child entity that holds the water mesh,
/// so the old water mesh can be replaced when the chunk is rebuilt.
#[derive(Component)]
pub struct ChunkWaterMesh(Entity);

/// Vertex data for one mesh (the solid blocks or the water).
#[derive(Default)]
struct MeshData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl MeshData {
    fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    fn into_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);
        mesh.insert_indices(Indices::U32(self.indices));
        mesh
    }
}

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
    query: Query<
        (Entity, &Chunk, Option<&Transform>, Option<&ChunkWaterMesh>),
        Changed<Chunk>,
    >,
) {
    for (entity, chunk, transform, old_water) in query.iter() {
        if !chunk.needs_remesh {
            continue;
        }

        // The mesh is drawn with the chunk entity's transform, so a block's
        // WORLD position is that translation plus the quad's local position.
        // Hashing the world position (not the chunk-local one) is what stops
        // every chunk from repeating the same pattern.
        let chunk_origin = transform.map(|t| t.translation).unwrap_or(Vec3::ZERO);

        let quads = generate_chunk_quads(chunk);

        // Solid blocks and water go in separate meshes, because water needs
        // the translucent material and everything else the opaque one.
        let mut solid = MeshData::default();
        let mut water = MeshData::default();

        for quad in &quads {
            let target = if matches!(quad.block_kind, BlockKind::Water) {
                &mut water
            } else {
                &mut solid
            };

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

            let base = target.positions.len() as u32;
            target.positions.extend_from_slice(&[
                v0.to_array(),
                v1.to_array(),
                v2.to_array(),
                v3.to_array(),
            ]);
            target.normals.extend_from_slice(&[quad.normal; 4]);

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
            target
                .uvs
                .extend_from_slice(&quad_uvs_for_hashed(quad.block_kind, face, hash));

            let tint = face_tint(hash);
            target.colors.extend_from_slice(&[[tint, tint, tint, 1.0]; 4]);

            target
                .indices
                .extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
        }

        // Solid blocks: the chunk entity itself, opaque material.
        commands.entity(entity).insert((
            Mesh3d(meshes.add(solid.into_mesh())),
            MeshMaterial3d(atlas.opaque.clone()),
        ));

        // Water: a child entity with the translucent material. Remove the old
        // one first so a rebuilt chunk never keeps stale water.
        if let Some(old) = old_water {
            commands.entity(old.0).despawn();
        }
        if water.is_empty() {
            commands.entity(entity).remove::<ChunkWaterMesh>();
        } else {
            let mut water_entity = None;
            commands.entity(entity).with_children(|parent| {
                water_entity = Some(
                    parent
                        .spawn((
                            Mesh3d(meshes.add(water.into_mesh())),
                            MeshMaterial3d(atlas.translucent.clone()),
                            Transform::default(),
                            NotShadowCaster, // water shouldn't darken the lake bed
                        ))
                        .id(),
                );
            });
            if let Some(id) = water_entity {
                commands.entity(entity).insert(ChunkWaterMesh(id));
            }
        }
    }
}