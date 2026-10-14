use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum BlockKind {
    Air,
    Stone,
    Dirt,
    Grass,
    Sand,
    Water,
    Wood,
    Leaves,
    Bedrock,
    CoalOre,
    IronOre,
    DiamondOre,
}

impl TryFrom<u8> for BlockKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Air),
            1 => Ok(Self::Stone),
            2 => Ok(Self::Dirt),
            3 => Ok(Self::Grass),
            4 => Ok(Self::Sand),
            5 => Ok(Self::Water),
            6 => Ok(Self::Wood),
            7 => Ok(Self::Leaves),
            8 => Ok(Self::Bedrock),
            9 => Ok(Self::CoalOre),
            10 => Ok(Self::IronOre),
            11 => Ok(Self::DiamondOre),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub struct Block {
    pub kind: BlockKind,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            kind: BlockKind::Air,
        }
    }
}

impl Block {
    pub const fn is_solid(&self) -> bool {
        matches!(
            self.kind,
            BlockKind::Stone
                | BlockKind::Dirt
                | BlockKind::Grass
                | BlockKind::Sand
                | BlockKind::Wood
                | BlockKind::Leaves
                | BlockKind::Bedrock
                | BlockKind::CoalOre
                | BlockKind::IronOre
                | BlockKind::DiamondOre
        )
    }

    pub const fn is_transparent(&self) -> bool {
        !self.is_solid() || matches!(self.kind, BlockKind::Water | BlockKind::Leaves)
    }

    pub const fn is_water(&self) -> bool {
        matches!(self.kind, BlockKind::Water)
    }

    /// Is this a grass block? Used by the mesher to skip grass faces when
    /// `assets/textures/grass.glb` exists and the model is rendered instead.
    ///
    /// Grass stays solid (see `is_solid`) so neighbouring blocks don't emit
    /// faces against it and the terrain stays watertight.
    pub const fn is_grass(&self) -> bool {
        matches!(self.kind, BlockKind::Grass)
    }

    /// Should the mesher create a face on the side of `self` that touches
    /// `neighbor`?
    ///
    /// - Air never has faces.
    /// - Water only shows a face where it meets air (the surface). Water next
    ///   to water, or water next to the ground, is hidden, so no face. This is
    ///   what removes the grid of walls inside lakes.
    /// - Everything else shows a face wherever the neighbor is see-through
    ///   (air, water, leaves), so the lake bed under the water is still drawn.
    pub const fn should_draw_face_against(&self, neighbor: &Block) -> bool {
        match self.kind {
            BlockKind::Air => false,
            BlockKind::Water => matches!(neighbor.kind, BlockKind::Air),
            _ => neighbor.is_transparent(),
        }
    }
}
