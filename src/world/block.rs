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
    // ── ores ─────────────────────────────────────────
    CoalOre,    // 9
    IronOre,    // 10
    DiamondOre, // 11
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
    // Future: orientation, state, etc.
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
}
