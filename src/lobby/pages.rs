use super::plugin::list_worlds;
use super::{
    GameMode, GameSettings, Generation, LobbyAction, LobbyPage, LobbyPanel, NameCursor,
    NewAdventureForm, ProgressFill, Selected, StatusText,
};
use bevy::prelude::*;

pub(super) const NORMAL: Color = Color::srgb(0.12, 0.25, 0.14);
pub(super) const HOVER: Color = Color::srgb(0.20, 0.38, 0.22);
pub(super) const SELECTED: Color = Color::srgb(0.42, 0.72, 0.28);

pub(super) const NAME_MAX_LEN: usize = 24;

/// Despawns the old page and builds the one in `LobbyPage`.
pub(super) fn rebuild_page(
    mut commands: Commands,
    page: Res<LobbyPage>,
    settings: Res<GameSettings>,
    form: Res<NewAdventureForm>,
    generation: Option<Res<Generation>>,
    old: Query<Entity, With<LobbyPanel>>,
) {
    for entity in &old {
        commands.entity(entity).despawn();
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(50.0), // menu on the left half; the island is on the right
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
            // No BackgroundColor: the 3D scene shows through.
            LobbyPanel,
        ))
        .with_children(|root| match *page {
            LobbyPage::Main => main_page(root),
            LobbyPage::NewAdventure => new_adventure_page(root, &form),
            LobbyPage::Generating => generating_page(root, generation.as_deref()),
            LobbyPage::Worlds => worlds_page(root),
            LobbyPage::Market => market_page(root),
            LobbyPage::Settings => settings_page(root, &settings),
        });
}

fn main_page(p: &mut ChildSpawnerCommands) {
    label(p, "Voxel Verse", 56.0);
    spacer(p);
    button(p, "New Adventure", LobbyAction::NewAdventure);
    button(p, "Recent Adventures", LobbyAction::GoTo(LobbyPage::Worlds));
    button(p, "Market", LobbyAction::GoTo(LobbyPage::Market));
    button(p, "Settings", LobbyAction::GoTo(LobbyPage::Settings));
}

fn new_adventure_page(p: &mut ChildSpawnerCommands, form: &NewAdventureForm) {
    label(p, "New Adventure", 40.0);
    spacer(p);

    label(p, "Adventure name", 20.0);
    p.spawn((
        Node {
            width: Val::Px(340.0),
            height: Val::Px(52.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.06, 0.14, 0.08)),
    ))
    .with_children(|b| {
        if form.name.is_empty() {
            cursor(b);
            label_colored(b, " Type a name...", 22.0, Color::srgb(0.55, 0.65, 0.55));
        } else {
            label(b, &form.name, 22.0);
            cursor(b);
        }
    });

    spacer(p);
    label(p, "Mode", 20.0);
    button_with(
        p,
        "Single Player",
        LobbyAction::SetMode(GameMode::SinglePlayer),
        form.mode == GameMode::SinglePlayer,
    );
    button_with(
        p,
        "Multiplayer",
        LobbyAction::SetMode(GameMode::Multiplayer),
        form.mode == GameMode::Multiplayer,
    );

    spacer(p);
    button(p, "Create Adventure", LobbyAction::CreateAdventure);
    back_button(p);
}

fn generating_page(p: &mut ChildSpawnerCommands, generation: Option<&Generation>) {
    label(p, "Generating World", 40.0);
    if let Some(g) = generation {
        label(p, &g.name, 24.0);
    }
    spacer(p);

    // Progress bar: an outer track with an inner fill that grows.
    p.spawn((
        Node {
            width: Val::Px(340.0),
            height: Val::Px(14.0),
            ..default()
        },
        BackgroundColor(NORMAL),
    ))
    .with_children(|bar| {
        bar.spawn((
            Node {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(SELECTED),
            ProgressFill,
        ));
    });

    p.spawn((
        Text::new("Preparing"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        StatusText,
    ));
}

fn worlds_page(p: &mut ChildSpawnerCommands) {
    label(p, "Recent Adventures", 40.0);
    spacer(p);

    let worlds = list_worlds();
    if worlds.is_empty() {
        label(p, "No adventures yet. Start a new one!", 20.0);
    }
    for (name, seed, multiplayer) in worlds.into_iter().take(8) {
        let text = if multiplayer {
            format!("Play {name} (Multiplayer)")
        } else {
            format!("Play {name}")
        };
        button(
            p,
            &text,
            LobbyAction::Play {
                name,
                seed,
                multiplayer,
            },
        );
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
    label_colored(p, text, size, Color::WHITE);
}

fn label_colored(p: &mut ChildSpawnerCommands, text: &str, size: f32, color: Color) {
    p.spawn((
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    ));
}

/// The blinking text cursor (blinked by `blink_cursor` in plugin.rs).
fn cursor(p: &mut ChildSpawnerCommands) {
    p.spawn((
        Text::new("|"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::WHITE),
        NameCursor,
    ));
}

fn button(p: &mut ChildSpawnerCommands, text: &str, action: LobbyAction) {
    button_with(p, text, action, false);
}

fn button_with(p: &mut ChildSpawnerCommands, text: &str, action: LobbyAction, selected: bool) {
    let mut e = p.spawn((
        Button,
        Node {
            width: Val::Px(340.0),
            height: Val::Px(52.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(if selected { SELECTED } else { NORMAL }),
        action,
    ));
    if selected {
        e.insert(Selected);
    }
    e.with_children(|b| label(b, text, 22.0));
}