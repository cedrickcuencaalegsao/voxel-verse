use super::{pages, AppState, GameSettings, LobbyAction, LobbyPage, LobbyPanel};
use crate::world::world_manager::WorldManager;
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SAVES_DIR: &str = "world/saves";

#[derive(Component)]
struct LobbyCamera;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<GameSettings>()
            .add_systems(OnEnter(AppState::Lobby), enter_lobby)
            .add_systems(OnExit(AppState::Lobby), exit_lobby)
            .add_systems(
                Update,
                (
                    pages::rebuild_page.run_if(resource_exists_and_changed::<LobbyPage>),
                    handle_buttons,
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
    mut buttons: Query<(&Interaction, &LobbyAction, &mut BackgroundColor), Changed<Interaction>>,
    mut page: ResMut<LobbyPage>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action, mut bg) in &mut buttons {
        match *interaction {
            Interaction::Pressed => match action {
                LobbyAction::GoTo(target) => *page = *target,
                LobbyAction::RenderDistance(delta) => {
                    settings.render_distance = (settings.render_distance + delta).clamp(2, 16);
                    page.set_changed(); // redraw the settings page
                }
                LobbyAction::NewAdventure => {
                    let (name, seed) = create_world();
                    start_game(&mut commands, &mut next_state, &name, seed);
                }
                LobbyAction::Play { name, seed } => {
                    start_game(&mut commands, &mut next_state, name, *seed);
                }
            },
            Interaction::Hovered => *bg = pages::HOVER.into(),
            Interaction::None => *bg = pages::NORMAL.into(),
        }
    }
}

fn start_game(commands: &mut Commands, next: &mut NextState<AppState>, name: &str, seed: u32) {
    // Inserted now so it already exists when the Playing state starts.
    commands.insert_resource(WorldManager::new(seed, name.to_string()));
    next.set(AppState::Playing);
}

/// Creates the next free `world_N` folder with a random seed.
fn create_world() -> (String, u32) {
    let root = PathBuf::from(SAVES_DIR);
    let mut n = 1;
    while root.join(format!("world_{n}")).exists() {
        n += 1;
    }
    let name = format!("world_{n}");
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32;

    let dir = root.join(&name);
    if let Err(e) =
        fs::create_dir_all(&dir).and_then(|_| fs::write(dir.join("seed.txt"), seed.to_string()))
    {
        eprintln!("Failed to create world: {e}");
    }
    (name, seed)
}

/// Every folder in world/saves as (name, seed). Worlds without a seed.txt
/// (like the old world_1) fall back to seed 42.
pub(super) fn list_worlds() -> Vec<(String, u32)> {
    let Ok(entries) = fs::read_dir(SAVES_DIR) else {
        return Vec::new();
    };
    let mut worlds: Vec<(String, u32)> = entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| {
            let seed = fs::read_to_string(e.path().join("seed.txt"))
                .ok()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(42);
            (e.file_name().to_string_lossy().to_string(), seed)
        })
        .collect();
    worlds.sort();
    worlds
}