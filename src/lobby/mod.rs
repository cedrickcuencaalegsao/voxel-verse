mod pages;
pub mod plugin;
mod scene;

use crate::utils::constants::RENDER_DISTANCE;
use bevy::prelude::*;

/// Top-level game state: the game opens in the lobby.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Lobby,
    Playing,
}

/// Which lobby screen is currently shown.
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum LobbyPage {
    Main,
    NewAdventure,
    Generating,
    Worlds,
    Market,
    Settings,
}

/// Single player or multiplayer session.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum GameMode {
    #[default]
    SinglePlayer,
    Multiplayer,
}

/// The "New Adventure" form (name + mode).
#[derive(Resource, Default)]
pub struct NewAdventureForm {
    pub name: String,
    pub mode: GameMode,
}

/// Exists while the world generation animation is running.
#[derive(Resource)]
pub struct Generation {
    pub timer: Timer,
    pub name: String,
    pub seed: u32,
    pub mode: GameMode,
}

/// Settings editable from the lobby.
#[derive(Resource)]
pub struct GameSettings {
    pub render_distance: i32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            render_distance: RENDER_DISTANCE,
        }
    }
}

/// Marker for the root UI node of the current lobby page.
#[derive(Component)]
pub struct LobbyPanel;

/// Marks everything that belongs to the 3D lobby backdrop
/// (camera, lights, island, character) so it can be despawned together.
#[derive(Component)]
pub struct LobbyScene;

/// Marks a button as the currently selected option (keeps its highlight).
#[derive(Component)]
pub struct Selected;

/// The filled part of the generation progress bar.
#[derive(Component)]
pub struct ProgressFill;

/// The status line under the progress bar.
#[derive(Component)]
pub struct StatusText;

/// The blinking cursor in the adventure name box.
#[derive(Component)]
pub struct NameCursor;

/// What a lobby button does when pressed.
#[derive(Component, Clone)]
pub enum LobbyAction {
    /// Opens the (reset) New Adventure form.
    NewAdventure,
    SetMode(GameMode),
    /// Creates the world folder and starts the generation animation.
    CreateAdventure,
    GoTo(LobbyPage),
    Play {
        name: String,
        seed: u32,
        multiplayer: bool,
    },
    RenderDistance(i32),
}