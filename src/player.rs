use bevy::{prelude::*, time::Stopwatch};
use std::time::Duration;

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
        app.add_startup_system(setup_playerinfo)
            .add_state(PlayerState::Playing)
            .add_system_set(SystemSet::on_enter(PlayerState::Playing).with_system(init_playerinfo))
            .add_system_set(SystemSet::on_update(PlayerState::Playing).with_system(tick_timer));
    }
}

#[derive(Component)]
pub struct PlayerInfo {
    play_timer: Stopwatch,
    move_count: usize,
}

impl PlayerInfo {
    pub fn new() -> Self {
        let mut stopwatch = Stopwatch::new();
        stopwatch.pause();
        Self {
            play_timer: stopwatch,
            move_count: 0,
        }
    }

    pub fn start_tracking(&mut self) {
        self.play_timer.unpause();
        self.move_count = 0;
    }

    pub fn add_move_count(&mut self) {
        if !self.play_timer.paused() {
            self.move_count += 1;
        }
    }

    pub fn reset(&mut self) {
        self.play_timer.pause();
        self.play_timer.reset();
        self.move_count = 0;
    }

    pub fn get_play_timer(&self) -> Duration {
        self.play_timer.elapsed()
    }

    pub fn get_move_count(&self) -> usize {
        self.move_count
    }
}

fn setup_playerinfo(mut commands: Commands) {
    commands
        .spawn(Name::new("PlayerInfo"))
        .insert(PlayerInfo::new());
}

fn init_playerinfo(mut player_info: Query<&mut PlayerInfo>) {
    player_info.single_mut().reset();
}

fn tick_timer(time: Res<Time>, mut player_info: Query<&mut PlayerInfo>) {
    let mut info = player_info.single_mut();
    info.play_timer.tick(time.delta());
}
