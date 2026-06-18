use crate::rendering::greedy_meshing::generate_chunk_quads;
use crate::rendering::materials::color_for_kind;
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
        [1.0, 0.0, 0.0] => (Vec3::Y, Vec3::Z),  // +X
        [-1.0, 0.0, 0.0] => (Vec3::Z, Vec3::Y), // -X
        [0.0, 1.0, 0.0] => (Vec3::Z, Vec3::X),  // +Y
        [0.0, -1.0, 0.0] => (Vec3::X, Vec3::Z), // -Y
        [0.0, 0.0, 1.0] => (Vec3::X, Vec3::Y),  // +Z
        [0.0, 0.0, -1.0] => (Vec3::Y, Vec3::X), // -Z
        _ => (Vec3::X, Vec3::Y),
    }
}

pub fn remesh_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut default_material: Local<Option<Handle<StandardMaterial>>>,
    query: Query<(Entity, &Chunk), Changed<Chunk>>,
) {
    let material_handle = default_material
        .get_or_insert_with(|| {
            materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            })
        })
        .clone();

    for (entity, chunk) in query.iter() {
        if !chunk.needs_remesh {
            continue;
        }

        let quads = generate_chunk_quads(chunk);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut colors = Vec::new();
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
            uvs.extend_from_slice(&[
                [quad.uv[0], quad.uv[1]],
                [quad.uv[2], quad.uv[1]],
                [quad.uv[2], quad.uv[3]],
                [quad.uv[0], quad.uv[3]],
            ]);
            let c = color_for_kind(quad.block_kind).to_linear().to_f32_array();
            colors.extend_from_slice(&[c; 4]);
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

        commands.entity(entity).insert((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material_handle.clone()),
        ));
    }
}
