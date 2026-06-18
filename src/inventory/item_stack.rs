use crate::world::block::BlockKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item: BlockKind,
    pub count: u8,
}

impl ItemStack {
    pub fn new(item: BlockKind, count: u8) -> Self {
        Self { item, count }
    }
}
