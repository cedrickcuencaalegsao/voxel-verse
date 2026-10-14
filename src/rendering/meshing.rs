use crate::models::grass_model::{GrassInstance, GrassModel};
use crate::models::leaves_model::{LeavesInstance, LeavesModel};
use crate::models::sand_model::{SandInstance, SandModel};
use crate::models::soil_model::{SoilInstance, SoilModel};
use crate::models::water_model::{WaterInstance, WaterModel};
use crate::models::wood_model::{WoodInstance, WoodModel};
use crate::rendering::atlas::{face_tint, quad_uvs_for_hashed};
use crate::rendering::greedy_meshing::generate_chunk_quads;
use crate::rendering::materials::{BlockAtlas, BlockFace};
use crate::rendering::texture::block_hash;
use crate::utils::constants::{CHUNK_HEIGHT, CHUNK_SIZE};
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

/// The model instances (grass, leaves, wood) spawned for a chunk, so they can
/// be despawned and rebuilt when the chunk is remeshed.
#[derive(Component)]
pub struct ChunkModels(Vec<Entity>);

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
        [1.0, 0.0, 0.0] => (Vec3::NEG_Z, Vec3::Y),  // +X
        [-1.0, 0.0, 0.0] => (Vec3::Z, Vec3::Y),     // -X
        [0.0, 0.0, 1.0] => (Vec3::X, Vec3::Y),      // +Z
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

/// True if at least one of the block's six neighbours is not solid, i.e. the
/// block can actually be seen. Neighbours outside this chunk count as exposed,
/// so blocks on a chunk border are always drawn (never a hole).
fn is_exposed(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    const DIRS: [(i32, i32, i32); 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let (cs, ch) = (CHUNK_SIZE as i32, CHUNK_HEIGHT as i32);

    for (dx, dy, dz) in DIRS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        let nz = z as i32 + dz;
        if nx < 0 || ny < 0 || nz < 0 || nx >= cs || ny >= ch || nz >= cs {
            return true;
        }
        if !chunk
            .get_block(nx as usize, ny as usize, nz as usize)
            .is_solid()
        {
            return true;
        }
    }
    false
}

/// Water is only visible where it touches air (the surface), exactly like
/// `Block::should_draw_face_against`. Water next to water or the ground is
/// hidden, so a lake gets one layer of instances instead of one per block.
/// Chunk borders are NOT treated as exposed here: the surface block is caught
/// by the "air above" test, and counting borders would spawn whole walls of
/// water along every chunk edge.
fn is_water_exposed(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
    const DIRS: [(i32, i32, i32); 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let (cs, ch) = (CHUNK_SIZE as i32, CHUNK_HEIGHT as i32);

    for (dx, dy, dz) in DIRS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        let nz = z as i32 + dz;
        if ny >= ch {
            return true; // open sky above the top of the chunk
        }
        if nx < 0 || ny < 0 || nz < 0 || nx >= cs || nz >= cs {
            continue;
        }
        if chunk.get_block(nx as usize, ny as usize, nz as usize).kind == BlockKind::Air {
            return true;
        }
    }
    false
}

pub fn remesh_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    atlas: Res<BlockAtlas>,
    grass_model: Res<GrassModel>,
    leaves_model: Res<LeavesModel>,
    wood_model: Res<WoodModel>,
    soil_model: Res<SoilModel>,
    sand_model: Res<SandModel>,
    water_model: Res<WaterModel>,
    query: Query<
        (
            Entity,
            &Chunk,
            Option<&Transform>,
            Option<&ChunkWaterMesh>,
            Option<&ChunkModels>,
        ),
        Changed<Chunk>,
    >,
) {
    // When a block kind has a loaded GLB, it is drawn as model instances and
    // left out of the chunk mesh. Otherwise the procedural textures are used.
    let scene_for = |kind: BlockKind| -> Option<&Handle<Scene>> {
        match kind {
            BlockKind::Grass => grass_model.0.as_ref(),
            BlockKind::Leaves => leaves_model.0.as_ref(),
            BlockKind::Wood => wood_model.0.as_ref(),
            BlockKind::Dirt => soil_model.0.as_ref(),
            BlockKind::Sand => sand_model.0.as_ref(),
            BlockKind::Water => water_model.0.as_ref(),
            _ => None,
        }
    };

    for (entity, chunk, transform, old_water, old_models) in query.iter() {
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
            // A GLB instance draws this block instead.
            if scene_for(quad.block_kind).is_some() {
                continue;
            }

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
            target
                .colors
                .extend_from_slice(&[[tint, tint, tint, 1.0]; 4]);

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

        // Models: remove the old instances, then spawn one GLB instance on
        // every visible grass / leaves / wood block of the rebuilt chunk.
        if let Some(old) = old_models {
            for &e in &old.0 {
                commands.entity(e).despawn();
            }
        }

        let mut ids = Vec::new();
        commands.entity(entity).with_children(|parent| {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let kind = chunk.get_block(x, y, z).kind;
                        let Some(scene) = scene_for(kind) else {
                            continue;
                        };

                        // Skip buried blocks (e.g. leaves inside the canopy).
                        // Water has its own rule: only the surface is drawn.
                        let visible = if kind == BlockKind::Water {
                            is_water_exposed(chunk, x, y, z)
                        } else {
                            is_exposed(chunk, x, y, z)
                        };
                        if !visible {
                            continue;
                        }

                        // Stable per-block rotation: hash the WORLD position so
                        // it never changes when the chunk is remeshed. The kind
                        // is mixed in so grass/leaves/wood rotate independently.
                        let wx = chunk_origin.x as i32 + x as i32;
                        let wy = chunk_origin.y as i32 + y as i32;
                        let wz = chunk_origin.z as i32 + z as i32;
                        let quarter_turns =
                            (block_hash(wx, wy, wz, 7 + kind as u32) as u64 % 4) as f32;
                        // Sand and water also get a random turn around the X axis
                        // (independent hash salt), on top of the Y turn. That gives
                        // 16 orientations instead of 4, so the same orientation
                        // rarely shows up twice in a row.
                        let x_turns = if matches!(kind, BlockKind::Sand | BlockKind::Water) {
                            (block_hash(wx, wy, wz, 31 + kind as u32) as u64 % 4) as f32
                        } else {
                            0.0
                        };
                        let rotation =
                            Quat::from_rotation_y(quarter_turns * std::f32::consts::FRAC_PI_2)
                                * Quat::from_rotation_x(x_turns * std::f32::consts::FRAC_PI_2);

                        let mut instance = parent.spawn((
                            SceneRoot(scene.clone()),
                            // Block (x,y,z) occupies [x,x+1]..., so +0.5 is its
                            // centre. Adjust if the GLB origin differs.
                            Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5)
                                .with_rotation(rotation),
                        ));
                        match kind {
                            BlockKind::Grass => {
                                instance.insert(GrassInstance);
                            }
                            BlockKind::Leaves => {
                                instance.insert(LeavesInstance);
                            }
                            BlockKind::Wood => {
                                instance.insert(WoodInstance);
                            }
                            BlockKind::Dirt => {
                                instance.insert(SoilInstance);
                            }
                            BlockKind::Sand => {
                                instance.insert(SandInstance);
                            }
                            BlockKind::Water => {
                                instance.insert(WaterInstance);
                            }
                            _ => {}
                        }
                        ids.push(instance.id());
                    }
                }
            }
        });

        if ids.is_empty() {
            commands.entity(entity).remove::<ChunkModels>();
        } else {
            commands.entity(entity).insert(ChunkModels(ids));
        }
    }
}
