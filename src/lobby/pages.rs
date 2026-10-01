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

// ── fonts ────────────────────────────────────────────────────────────────────

/// Font handles for the lobby UI. Register with `app.init_resource::<UiFonts>()`
/// in the lobby plugin. Paths are relative to the `assets/` folder and
/// case-sensitive, so make sure the folder name matches what is on disk.
#[derive(Resource)]
pub(super) struct UiFonts {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
}

impl FromWorld for UiFonts {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            regular: assets.load("fonts/pexilfy_sans/PixelifySans-Regular.ttf"),
            bold: assets.load("fonts/pexilfy_sans/PixelifySans-Bold.ttf"),
        }
    }
}

// ── page root ────────────────────────────────────────────────────────────────

/// Despawns the old page and builds the one in `LobbyPage`.
pub(super) fn rebuild_page(
    mut commands: Commands,
    page: Res<LobbyPage>,
    settings: Res<GameSettings>,
    form: Res<NewAdventureForm>,
    fonts: Res<UiFonts>,
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
            LobbyPage::Main => main_page(root, &fonts),
            LobbyPage::NewAdventure => new_adventure_page(root, &form, &fonts),
            LobbyPage::Generating => generating_page(root, generation.as_deref(), &fonts),
            LobbyPage::Worlds => worlds_page(root, &fonts),
            LobbyPage::Market => market_page(root, &fonts),
            LobbyPage::Settings => settings_page(root, &settings, &fonts),
        });
}

// ── pages ────────────────────────────────────────────────────────────────────

fn main_page(p: &mut ChildSpawnerCommands, f: &UiFonts) {
    title(p, f, "Voxel Verse", 56.0);
    spacer(p);
    button(p, f, "New Adventure", LobbyAction::NewAdventure);
    button(p, f, "Recent Adventures", LobbyAction::GoTo(LobbyPage::Worlds));
    button(p, f, "Market", LobbyAction::GoTo(LobbyPage::Market));
    button(p, f, "Settings", LobbyAction::GoTo(LobbyPage::Settings));
}

fn new_adventure_page(p: &mut ChildSpawnerCommands, form: &NewAdventureForm, f: &UiFonts) {
    title(p, f, "New Adventure", 40.0);
    spacer(p);

    label(p, f, "Adventure name", 20.0);
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
            cursor(b, f);
            label_colored(b, f, " Type a name...", 22.0, Color::srgb(0.55, 0.65, 0.55));
        } else {
            label(b, f, &form.name, 22.0);
            cursor(b, f);
        }
    });

    spacer(p);
    label(p, f, "Mode", 20.0);
    button_with(
        p,
        f,
        "Single Player",
        LobbyAction::SetMode(GameMode::SinglePlayer),
        form.mode == GameMode::SinglePlayer,
    );
    button_with(
        p,
        f,
        "Multiplayer",
        LobbyAction::SetMode(GameMode::Multiplayer),
        form.mode == GameMode::Multiplayer,
    );

    spacer(p);
    button(p, f, "Create Adventure", LobbyAction::CreateAdventure);
    back_button(p, f);
}

fn generating_page(p: &mut ChildSpawnerCommands, generation: Option<&Generation>, f: &UiFonts) {
    title(p, f, "Generating World", 40.0);
    if let Some(g) = generation {
        label(p, f, &g.name, 24.0);
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
            font: f.regular.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        StatusText,
    ));
}

fn worlds_page(p: &mut ChildSpawnerCommands, f: &UiFonts) {
    title(p, f, "Recent Adventures", 40.0);
    spacer(p);

    let worlds = list_worlds();
    if worlds.is_empty() {
        label(p, f, "No adventures yet. Start a new one!", 20.0);
    }
    for (name, seed, multiplayer) in worlds.into_iter().take(8) {
        let text = if multiplayer {
            format!("Play {name} (Multiplayer)")
        } else {
            format!("Play {name}")
        };
        button(
            p,
            f,
            &text,
            LobbyAction::Play {
                name,
                seed,
                multiplayer,
            },
        );
    }

    spacer(p);
    back_button(p, f);
}

fn market_page(p: &mut ChildSpawnerCommands, f: &UiFonts) {
    title(p, f, "Market", 40.0);
    label(p, f, "Coming soon", 20.0);
    spacer(p);
    back_button(p, f);
}

fn settings_page(p: &mut ChildSpawnerCommands, settings: &GameSettings, f: &UiFonts) {
    title(p, f, "Settings", 40.0);
    spacer(p);
    label(
        p,
        f,
        &format!("Render distance: {} chunks", settings.render_distance),
        22.0,
    );
    button(
        p,
        f,
        "Increase render distance",
        LobbyAction::RenderDistance(1),
    );
    button(
        p,
        f,
        "Decrease render distance",
        LobbyAction::RenderDistance(-1),
    );
    spacer(p);
    back_button(p, f);
}

// ── small UI helpers ─────────────────────────────────────────────────────────

fn back_button(p: &mut ChildSpawnerCommands, f: &UiFonts) {
    button(p, f, "Back", LobbyAction::GoTo(LobbyPage::Main));
}

fn spacer(p: &mut ChildSpawnerCommands) {
    p.spawn(Node {
        height: Val::Px(16.0),
        ..default()
    });
}

/// Page titles use the bold weight.
fn title(p: &mut ChildSpawnerCommands, f: &UiFonts, text: &str, size: f32) {
    p.spawn((
        Text::new(text),
        TextFont {
            font: f.bold.clone(),
            font_size: size,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn label(p: &mut ChildSpawnerCommands, f: &UiFonts, text: &str, size: f32) {
    label_colored(p, f, text, size, Color::WHITE);
}

fn label_colored(p: &mut ChildSpawnerCommands, f: &UiFonts, text: &str, size: f32, color: Color) {
    p.spawn((
        Text::new(text),
        TextFont {
            font: f.regular.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
    ));
}

/// The blinking text cursor (blinked by `blink_cursor` in plugin.rs).
fn cursor(p: &mut ChildSpawnerCommands, f: &UiFonts) {
    p.spawn((
        Text::new("|"),
        TextFont {
            font: f.regular.clone(),
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::WHITE),
        NameCursor,
    ));
}

fn button(p: &mut ChildSpawnerCommands, f: &UiFonts, text: &str, action: LobbyAction) {
    button_with(p, f, text, action, false);
}

fn button_with(
    p: &mut ChildSpawnerCommands,
    f: &UiFonts,
    text: &str,
    action: LobbyAction,
    selected: bool,
) {
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
    e.with_children(|b| label(b, f, text, 22.0));
}