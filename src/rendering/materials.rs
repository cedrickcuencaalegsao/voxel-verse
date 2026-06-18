use crate::world::block::BlockKind;
use bevy::prelude::*;

/// Flat placeholder color per block kind, used for chunk-mesh vertex
/// coloring until a real texture atlas is wired up.
pub fn color_for_kind(kind: BlockKind) -> Color {
    match kind {
        BlockKind::Air => Color::srgba(0.0, 0.0, 0.0, 0.0),
        BlockKind::Stone => Color::srgb(0.5, 0.5, 0.5),
        BlockKind::Dirt => Color::srgb(0.6, 0.4, 0.2),
        BlockKind::Grass => Color::srgb(0.2, 0.8, 0.2),
        BlockKind::Sand => Color::srgb(0.9, 0.8, 0.5),
        BlockKind::Water => Color::srgba(0.1, 0.2, 0.9, 0.5),
        BlockKind::Wood => Color::srgb(0.4, 0.2, 0.0),
        BlockKind::Leaves => Color::srgb(0.1, 0.8, 0.1),
        BlockKind::Bedrock => Color::srgb(0.1, 0.1, 0.1),
    }
}
