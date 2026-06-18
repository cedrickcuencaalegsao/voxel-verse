use super::item_stack::ItemStack;
use bevy::prelude::*;

pub const INVENTORY_SLOTS: usize = 36; // includes hotbar
pub const HOTBAR_SLOTS: usize = 9;

#[derive(Resource)]
pub struct Inventory {
    pub slots: [Option<ItemStack>; INVENTORY_SLOTS],
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: [None; INVENTORY_SLOTS], // all slots are None
        }
    }
}

impl Inventory {
    pub fn hotbar(&self) -> &[Option<ItemStack>] {
        &self.slots[0..HOTBAR_SLOTS]
    }

    pub fn hotbar_mut(&mut self) -> &mut [Option<ItemStack>] {
        &mut self.slots[0..HOTBAR_SLOTS]
    }
}
