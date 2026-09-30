mod pages;
pub mod plugin;

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
    Worlds,
    Market,
    Settings,
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

/// What a lobby button does when pressed.
#[derive(Component, Clone)]
pub enum LobbyAction {
    NewAdventure,
    GoTo(LobbyPage),
    Play { name: String, seed: u32 },
    RenderDistance(i32),
}