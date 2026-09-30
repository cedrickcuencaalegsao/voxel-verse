use super::plugin::list_worlds;
use super::{GameSettings, LobbyAction, LobbyPage, LobbyPanel};
use bevy::prelude::*;

pub(super) const NORMAL: Color = Color::srgb(0.20, 0.22, 0.30);
pub(super) const HOVER: Color = Color::srgb(0.30, 0.34, 0.48);

/// Despawns the old page and builds the one in `LobbyPage`.
pub(super) fn rebuild_page(
    mut commands: Commands,
    page: Res<LobbyPage>,
    settings: Res<GameSettings>,
    old: Query<Entity, With<LobbyPanel>>,
) {
    for entity in &old {
        commands.entity(entity).despawn();
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.07, 0.08, 0.12)),
            LobbyPanel,
        ))
        .with_children(|root| match *page {
            LobbyPage::Main => main_page(root),
            LobbyPage::Worlds => worlds_page(root),
            LobbyPage::Market => market_page(root),
            LobbyPage::Settings => settings_page(root, &settings),
        });
}

fn main_page(p: &mut ChildSpawnerCommands) {
    label(p, "Voxel Verse", 56.0);
    spacer(p);
    button(p, "New Adventure", LobbyAction::NewAdventure);
    button(p, "Worlds", LobbyAction::GoTo(LobbyPage::Worlds));
    button(p, "Market", LobbyAction::GoTo(LobbyPage::Market));
    button(p, "Settings", LobbyAction::GoTo(LobbyPage::Settings));
}

fn worlds_page(p: &mut ChildSpawnerCommands) {
    label(p, "Your Worlds", 40.0);
    spacer(p);

    let worlds = list_worlds();
    if worlds.is_empty() {
        label(p, "No worlds yet. Start a new adventure!", 20.0);
    }
    for (name, seed) in worlds.into_iter().take(8) {
        button(p, &format!("Play {name}"), LobbyAction::Play { name, seed });
    }

    spacer(p);
    back_button(p);
}

fn market_page(p: &mut ChildSpawnerCommands) {
    label(p, "Market", 40.0);
    label(p, "Coming soon", 20.0);
    spacer(p);
    back_button(p);
}

fn settings_page(p: &mut ChildSpawnerCommands, settings: &GameSettings) {
    label(p, "Settings", 40.0);
    spacer(p);
    label(
        p,
        &format!("Render distance: {} chunks", settings.render_distance),
        22.0,
    );
    button(p, "Increase render distance", LobbyAction::RenderDistance(1));
    button(p, "Decrease render distance", LobbyAction::RenderDistance(-1));
    spacer(p);
    back_button(p);
}

// ── small UI helpers ─────────────────────────────────────────────────────────

fn back_button(p: &mut ChildSpawnerCommands) {
    button(p, "Back", LobbyAction::GoTo(LobbyPage::Main));
}

fn spacer(p: &mut ChildSpawnerCommands) {
    p.spawn(Node {
        height: Val::Px(16.0),
        ..default()
    });
}

fn label(p: &mut ChildSpawnerCommands, text: &str, size: f32) {
    p.spawn((
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn button(p: &mut ChildSpawnerCommands, text: &str, action: LobbyAction) {
    p.spawn((
        Button,
        Node {
            width: Val::Px(340.0),
            height: Val::Px(52.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL),
        action,
    ))
    .with_children(|b| label(b, text, 22.0));
}