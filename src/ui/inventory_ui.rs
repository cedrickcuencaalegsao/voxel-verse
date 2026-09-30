use bevy::prelude::*;

pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_inventory);
    }
}

#[derive(Component)]
pub struct InventoryWindow;

fn toggle_inventory(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<InventoryWindow>>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        if let Ok(entity) = query.single() {
            // Inventory is currently open, so close it.
            // In Bevy 0.16+, despawn() is automatically recursive.
            commands.entity(entity).despawn();
        } else {
            // Inventory is closed, so spawn it!
            spawn_inventory_ui(&mut commands);
        }
    }
}

fn spawn_inventory_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(25.0),
                right: Val::Percent(25.0),
                top: Val::Percent(20.0),
                bottom: Val::Percent(20.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15).with_alpha(0.95)),
            GlobalZIndex(300),
            InventoryWindow,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PLAYER INVENTORY"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            // Grid of inventory slots
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::px(6, 48.0),
                    grid_template_rows: RepeatedGridTrack::px(4, 48.0),
                    row_gap: Val::Px(4.0),
                    column_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|grid| {
                    for i in 0..24 {
                        grid.spawn((
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(48.0),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                            BackgroundColor(Color::srgb(0.3, 0.3, 0.35)),
                        ))
                        .with_children(|slot| {
                            slot.spawn((
                                Text::new(format!("{}", i)),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            ));
                        });
                    }
                });

            // Close hint
            parent.spawn((
                Text::new("Press 'Q' to Close"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}
