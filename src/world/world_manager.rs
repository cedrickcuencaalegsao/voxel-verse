use super::generator;
use super::terrain::TerrainGenerator;
use crate::utils::constants::{CHUNK_HEIGHT, CHUNK_SIZE, RENDER_DISTANCE};
use crate::utils::math::chunk_pos_to_idx;
use crate::world::load::load_chunk;
use crate::world::save::save_chunk;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct WorldManager {
    pub generator: TerrainGenerator,
    pub chunk_entities: HashMap<u64, Entity>,
}

impl WorldManager {
    pub fn new(seed: u32) -> Self {
        Self {
            generator: TerrainGenerator::new(seed),
            chunk_entities: HashMap::new(),
        }
    }

}

/// Marker component for the player
#[derive(Component)]
pub struct Player;

/// System that ensures chunks around the player are spawned
pub fn chunk_streaming_system(
    mut commands: Commands,
    mut world: ResMut<WorldManager>,
    player_query: Query<&Transform, With<Player>>,
) {
    // Get the player position
    let player_transform = match player_query.iter().next() {
        Some(t) => t,
        None => return, // no player yet, do nothing
    };

    let player_pos = player_transform.translation;
    let player_chunk = IVec3::new(
        (player_pos.x / 16.0).floor() as i32,
        (player_pos.y / 16.0).floor() as i32,
        (player_pos.z / 16.0).floor() as i32,
    );

    let radius = RENDER_DISTANCE;
    let mut desired_chunks = Vec::new();

    // Destructure WorldManager so we can borrow generator and chunk_entities separately
    let WorldManager {
        generator,
        chunk_entities,
    } = &mut *world;

    // Determine which chunks are missing
    for x in (player_chunk.x - radius)..=(player_chunk.x + radius) {
        for z in (player_chunk.z - radius)..=(player_chunk.z + radius) {
            let chunk_pos = IVec3::new(x, 0, z); // fixed height layer for now
            let idx = chunk_pos_to_idx(chunk_pos.x, chunk_pos.y, chunk_pos.z);
            if !chunk_entities.contains_key(&idx) {
                desired_chunks.push(chunk_pos);
            }
        }
    }

    // Load or generate each missing chunk, then spawn it
    for chunk_pos in desired_chunks {
        let chunk = match load_chunk("world_1", chunk_pos) {
            Some(chunk) => chunk,
            None => {
                let new_chunk = generator::generate_chunk(chunk_pos, generator);
                if let Err(e) = save_chunk("world_1", &new_chunk) {
                    eprintln!("Failed to save chunk: {}", e);
                }
                new_chunk
            }
        };

        let world_pos = Vec3::new(
            chunk_pos.x as f32 * CHUNK_SIZE as f32,
            chunk_pos.y as f32 * CHUNK_HEIGHT as f32,
            chunk_pos.z as f32 * CHUNK_SIZE as f32,
        );

        let entity = commands
            .spawn((
                chunk,
                Transform::from_translation(world_pos),
                Visibility::default(),
            ))
            .id();

        let idx = chunk_pos_to_idx(chunk_pos.x, chunk_pos.y, chunk_pos.z);
        chunk_entities.insert(idx, entity);
    }
}
