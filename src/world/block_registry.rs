use super::block::BlockKind;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::collections::HashMap;

#[derive(Resource)]
pub struct BlockRegistry {
    pub meshes: HashMap<BlockKind, Handle<Mesh>>,
    pub materials: HashMap<BlockKind, Handle<StandardMaterial>>,
    pub texture_atlas: Option<Handle<Image>>,
}

impl BlockRegistry {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> Self {
        let mut reg = Self {
            meshes: HashMap::new(),
            materials: HashMap::new(),
            texture_atlas: None,
        };

        let atlas_image = Image::new_fill(
            Extent3d {
                width: 16,
                height: 16,
                ..Default::default()
            },
            TextureDimension::D2,
            &[255u8; 16 * 16 * 4],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        let atlas_handle = images.add(atlas_image);
        reg.texture_atlas = Some(atlas_handle.clone());

        for kind in &[
            BlockKind::Stone,
            BlockKind::Dirt,
            BlockKind::Grass,
            BlockKind::Sand,
            BlockKind::Water,
            BlockKind::Wood,
            BlockKind::Leaves,
            BlockKind::Bedrock,
            BlockKind::CoalOre,
            BlockKind::IronOre,
            BlockKind::DiamondOre,
        ] {
            let (mesh, mat) = reg.create_block_assets(*kind, meshes, materials, &atlas_handle);
            reg.meshes.insert(*kind, mesh);
            reg.materials.insert(*kind, mat);
        }

        reg
    }

    fn create_block_assets(
        &self,
        kind: BlockKind,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        texture: &Handle<Image>,
    ) -> (Handle<Mesh>, Handle<StandardMaterial>) {
        let mesh_handle = meshes.add(create_unit_cube_mesh());

        let color = match kind {
            BlockKind::Stone => Color::srgb(0.5, 0.5, 0.5),
            BlockKind::Dirt => Color::srgb(0.6, 0.4, 0.2),
            BlockKind::Grass => Color::srgb(0.2, 0.8, 0.2),
            BlockKind::Sand => Color::srgb(0.9, 0.8, 0.5),
            BlockKind::Water => Color::srgba(0.1, 0.2, 0.9, 0.5),
            BlockKind::Wood => Color::srgb(0.4, 0.2, 0.0),
            BlockKind::Leaves => Color::srgb(0.1, 0.8, 0.1),
            BlockKind::Bedrock => Color::srgb(0.1, 0.1, 0.1),
            BlockKind::CoalOre => Color::srgb(0.25, 0.25, 0.25),
            BlockKind::IronOre => Color::srgb(0.72, 0.55, 0.40),
            BlockKind::DiamondOre => Color::srgb(0.3, 0.85, 0.9),
            BlockKind::Air => Color::srgba(0.0, 0.0, 0.0, 0.0),
        };

        let mat = materials.add(StandardMaterial {
            base_color: color,
            base_color_texture: Some(texture.clone()),
            ..default()
        });

        (mesh_handle, mat)
    }
}

fn create_unit_cube_mesh() -> Mesh {
    Mesh::from(Cuboid::new(1.0, 1.0, 1.0))
}

pub fn init_block_registry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.insert_resource(BlockRegistry::new(&mut meshes, &mut materials, &mut images));
}
