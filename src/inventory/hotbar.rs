use super::inventory::Inventory;
use crate::world::block::BlockKind;
use bevy::prelude::*;

pub fn hotbar_input(keys: Res<ButtonInput<KeyCode>>, mut inventory: ResMut<Inventory>) {
    for key in 1..=9 {
        let keycode = match key {
            1 => KeyCode::Digit1,
            2 => KeyCode::Digit2,
            3 => KeyCode::Digit3,
            4 => KeyCode::Digit4,
            5 => KeyCode::Digit5,
            6 => KeyCode::Digit6,
            7 => KeyCode::Digit7,
            8 => KeyCode::Digit8,
            9 => KeyCode::Digit9,
            _ => continue,
        };
        if keys.just_pressed(keycode) {
            // Just for now, add a stone block to slot (key-1) to test
            let slot = (key - 1) as usize;
            inventory.hotbar_mut()[slot] =
                Some(super::item_stack::ItemStack::new(BlockKind::Stone, 64));
        }
    }
}
