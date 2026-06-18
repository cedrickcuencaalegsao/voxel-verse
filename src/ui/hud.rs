use crate::inventory::inventory::Inventory;
use bevy::prelude::*;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Percent(50.0),
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            for i in 0..9 {
                parent.spawn((
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(48.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::horizontal(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    HotbarSlot(i),
                ));
            }
        });
}

#[derive(Component)]
pub struct HotbarSlot(pub usize);

pub fn update_hotbar(
    inventory: Res<Inventory>,
    mut query: Query<(&HotbarSlot, &mut BackgroundColor)>,
) {
    for (slot, mut bg) in query.iter_mut() {
        if let Some(item) = &inventory.hotbar()[slot.0] {
            // Color based on block kind
            let color = match item.item {
                crate::world::block::BlockKind::Stone => Color::srgb(0.5, 0.5, 0.5),
                crate::world::block::BlockKind::Dirt => Color::srgb(0.6, 0.4, 0.2),
                crate::world::block::BlockKind::Grass => Color::srgb(0.2, 0.8, 0.2),
                _ => Color::srgb(1.0, 1.0, 1.0),
            };
            *bg = BackgroundColor(color.with_alpha(item.count as f32 / 64.0));
        } else {
            *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
        }
    }
}
