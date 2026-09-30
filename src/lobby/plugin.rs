use super::{
    pages, AppState, GameMode, GameSettings, Generation, LobbyAction, LobbyPage, LobbyPanel,
    NameCursor, NewAdventureForm, ProgressFill, Selected, StatusText,
};
use crate::world::world_manager::WorldManager;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SAVES_DIR: &str = "world/saves";

/// The generation animation runs at least this long.
const GENERATION_SECONDS: f32 = 3.0;

/// Cursor blinks this many times per second.
const CURSOR_BLINKS_PER_SECOND: f32 = 2.0;

#[derive(Component)]
struct LobbyCamera;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<GameSettings>()
            .init_resource::<NewAdventureForm>()
            .add_systems(OnEnter(AppState::Lobby), enter_lobby)
            .add_systems(OnExit(AppState::Lobby), exit_lobby)
            .add_systems(
                Update,
                (
                    pages::rebuild_page.run_if(resource_exists_and_changed::<LobbyPage>),
                    handle_buttons,
                    name_input.run_if(resource_exists_and_equals(LobbyPage::NewAdventure)),
                    blink_cursor.run_if(resource_exists_and_equals(LobbyPage::NewAdventure)),
                    animate_generation.run_if(resource_exists::<Generation>),
                )
                    .chain()
                    .run_if(in_state(AppState::Lobby)),
            );
    }
}

fn enter_lobby(mut commands: Commands) {
    commands.spawn((Camera2d, LobbyCamera));
    // Inserting the resource counts as a change, which builds the first page.
    commands.insert_resource(LobbyPage::Main);
}

fn exit_lobby(
    mut commands: Commands,
    query: Query<Entity, Or<(With<LobbyPanel>, With<LobbyCamera>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_buttons(
    mut commands: Commands,
    mut buttons: Query<
        (&Interaction, &LobbyAction, &mut BackgroundColor, Has<Selected>),
        Changed<Interaction>,
    >,
    mut page: ResMut<LobbyPage>,
    mut settings: ResMut<GameSettings>,
    mut form: ResMut<NewAdventureForm>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action, mut bg, selected) in &mut buttons {
        match *interaction {
            Interaction::Pressed => match action {
                LobbyAction::GoTo(target) => *page = *target,
                LobbyAction::RenderDistance(delta) => {
                    settings.render_distance = (settings.render_distance + delta).clamp(2, 16);
                    page.set_changed(); // redraw the settings page
                }
                LobbyAction::NewAdventure => {
                    *form = NewAdventureForm::default();
                    *page = LobbyPage::NewAdventure;
                }
                LobbyAction::SetMode(mode) => {
                    form.mode = *mode;
                    page.set_changed(); // redraw so the highlight moves
                }
                LobbyAction::CreateAdventure => {
                    let (name, seed) = create_world(&form.name, form.mode);
                    commands.insert_resource(Generation {
                        timer: Timer::from_seconds(GENERATION_SECONDS, TimerMode::Once),
                        name,
                        seed,
                        mode: form.mode,
                    });
                    *page = LobbyPage::Generating;
                }
                LobbyAction::Play {
                    name,
                    seed,
                    multiplayer,
                } => {
                    let mode = if *multiplayer {
                        GameMode::Multiplayer
                    } else {
                        GameMode::SinglePlayer
                    };
                    start_game(&mut commands, &mut next_state, name, *seed, mode);
                }
            },
            Interaction::Hovered => *bg = pages::HOVER.into(),
            Interaction::None => {
                *bg = if selected {
                    pages::SELECTED.into()
                } else {
                    pages::NORMAL.into()
                }
            }
        }
    }
}

/// Types into the adventure name field.
fn name_input(
    mut events: MessageReader<KeyboardInput>,
    mut form: ResMut<NewAdventureForm>,
    mut page: ResMut<LobbyPage>,
) {
    let mut changed = false;
    for ev in events.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        match &ev.logical_key {
            Key::Backspace => changed |= form.name.pop().is_some(),
            Key::Space => {
                if form.name.chars().count() < pages::NAME_MAX_LEN {
                    form.name.push(' ');
                    changed = true;
                }
            }
            Key::Character(s) => {
                for c in s.chars() {
                    let allowed = c.is_alphanumeric() || c == '-' || c == '_';
                    if allowed && form.name.chars().count() < pages::NAME_MAX_LEN {
                        form.name.push(c);
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    if changed {
        page.set_changed(); // redraw the name box
    }
}

/// Blinks the name cursor. Only the alpha changes, so the text never shifts.
fn blink_cursor(time: Res<Time>, mut cursors: Query<&mut TextColor, With<NameCursor>>) {
    let on = (time.elapsed_secs() * CURSOR_BLINKS_PER_SECOND) as u32 % 2 == 0;
    let alpha = if on { 1.0 } else { 0.0 };
    for mut color in &mut cursors {
        color.0 = Color::srgba(1.0, 1.0, 1.0, alpha);
    }
}

/// Fills the progress bar, cycles the status text, and starts the game when done.
fn animate_generation(
    mut commands: Commands,
    time: Res<Time>,
    mut generation: ResMut<Generation>,
    mut fill: Query<&mut Node, With<ProgressFill>>,
    mut status: Query<&mut Text, With<StatusText>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    generation.timer.tick(time.delta());
    let t = generation.timer.fraction();

    for mut node in &mut fill {
        node.width = Val::Percent(t * 100.0);
    }

    let stage = match t {
        t if t < 0.25 => "Shaping terrain",
        t if t < 0.50 => "Carving caves",
        t if t < 0.75 => "Planting trees",
        _ => "Placing final touches",
    };
    let dots = ".".repeat((generation.timer.elapsed_secs() * 3.0) as usize % 4);
    for mut text in &mut status {
        text.0 = format!("{stage}{dots}");
    }

    if generation.timer.is_finished() {
        start_game(
            &mut commands,
            &mut next_state,
            &generation.name,
            generation.seed,
            generation.mode,
        );
        commands.remove_resource::<Generation>();
    }
}

fn start_game(
    commands: &mut Commands,
    next: &mut NextState<AppState>,
    name: &str,
    seed: u32,
    mode: GameMode,
) {
    // Inserted now so they already exist when the Playing state starts.
    commands.insert_resource(WorldManager::new(seed, name.to_string()));
    commands.insert_resource(mode);
    next.set(AppState::Playing);
}

/// Creates the world folder from the player's name (made filesystem-safe and
/// unique) with a random seed.
fn create_world(raw_name: &str, mode: GameMode) -> (String, u32) {
    let cleaned: String = raw_name
        .trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_'))
        .collect();
    let base = if cleaned.trim().is_empty() {
        "Adventure".to_string()
    } else {
        cleaned.trim().to_string()
    };

    let root = PathBuf::from(SAVES_DIR);
    let mut name = base.clone();
    let mut n = 2;
    while root.join(&name).exists() {
        name = format!("{base} {n}");
        n += 1;
    }

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32;

    let dir = root.join(&name);
    let mode_str = if mode == GameMode::Multiplayer {
        "multiplayer"
    } else {
        "singleplayer"
    };
    if let Err(e) = fs::create_dir_all(&dir)
        .and_then(|_| fs::write(dir.join("seed.txt"), seed.to_string()))
        .and_then(|_| fs::write(dir.join("mode.txt"), mode_str))
    {
        eprintln!("Failed to create world: {e}");
    }
    (name, seed)
}

/// Every folder in world/saves as (name, seed, multiplayer). Worlds without a
/// seed.txt fall back to seed 42; without a mode.txt they're single player.
pub(super) fn list_worlds() -> Vec<(String, u32, bool)> {
    let Ok(entries) = fs::read_dir(SAVES_DIR) else {
        return Vec::new();
    };
    let mut worlds: Vec<(String, u32, bool)> = entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| {
            let seed = fs::read_to_string(e.path().join("seed.txt"))
                .ok()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(42);
            let multiplayer = fs::read_to_string(e.path().join("mode.txt"))
                .map(|s| s.trim() == "multiplayer")
                .unwrap_or(false);
            (e.file_name().to_string_lossy().to_string(), seed, multiplayer)
        })
        .collect();
    worlds.sort();
    worlds
}