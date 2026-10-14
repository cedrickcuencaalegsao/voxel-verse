use bevy::prelude::*;
use std::path::PathBuf;

/// `Some(handle)` when assets/textures/sand.glb exists, otherwise `None`
/// (the procedural sand textures are used then).
#[derive(Resource, Default)]
pub struct SandModel(pub Option<Handle<Scene>>);

/// Marker on every spawned sand instance.
#[derive(Component)]
pub struct SandInstance;

pub fn load_sand_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let path: PathBuf = [root.as_str(), "assets", "textures", "sand.glb"]
        .iter()
        .collect();

    let handle = path
        .exists()
        .then(|| asset_server.load("textures/sand.glb#Scene0"));

    if handle.is_none() {
        info!("sand.glb not found, using procedural sand textures");
    }
    commands.insert_resource(SandModel(handle));
}
