use bevy::prelude::*;
use std::path::PathBuf;

/// `Some(handle)` when assets/textures/grass.glb exists, otherwise `None`
/// (the procedural gen_grass_top / gen_grass_side textures are used then).
#[derive(Resource, Default)]
pub struct GrassModel(pub Option<Handle<Scene>>);

/// Marker on every spawned grass instance.
#[derive(Component)]
pub struct GrassInstance;

pub fn load_grass_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let path: PathBuf = [root.as_str(), "assets", "textures", "grass.glb"]
        .iter()
        .collect();

    let handle = path
        .exists()
        .then(|| asset_server.load("textures/grass.glb#Scene0"));

    if handle.is_none() {
        info!("grass.glb not found, using procedural grass textures");
    }
    commands.insert_resource(GrassModel(handle));
}
