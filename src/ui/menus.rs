use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_settings_menu);
    }
}

#[derive(Component)]
pub struct SettingsMenuWindow;

fn toggle_settings_menu(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<SettingsMenuWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok(entity) = query.single() {
            // Close menu.
            // In Bevy 0.16+, despawn() is automatically recursive.
            commands.entity(entity).despawn();
        } else {
            // Open menu
            spawn_settings_menu(&mut commands);
        }
    }
}

fn spawn_settings_menu(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(30.0),
                right: Val::Percent(30.0),
                top: Val::Percent(25.0),
                bottom: Val::Percent(25.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08).with_alpha(0.98)),
            GlobalZIndex(400),
            SettingsMenuWindow,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SETTINGS & OPTIONS"),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Setting Item 1
            parent.spawn(Node::default()).with_children(|row| {
                row.spawn((
                    Text::new("RENDER DISTANCE: "),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                ));
                row.spawn((
                    Text::new("12 CHUNKS"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 0.2)),
                ));
            });

            // Setting Item 2
            parent.spawn(Node::default()).with_children(|row| {
                row.spawn((
                    Text::new("MOUSE SENSITIVITY: "),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                ));
                row.spawn((
                    Text::new("0.003"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 0.2)),
                ));
            });

            // Help instruction
            parent.spawn((
                Text::new("Press 'ESC' to Close Settings"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}
