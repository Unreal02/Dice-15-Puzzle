use bevy::prelude::{App, Plugin};

/// PlayerState represent state shift of player from game start to end
/// So, PlayerPlugin would control such state transitions of player.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
    Title,
    Playing,
    GameClear,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerState::Playing);
    }
}
