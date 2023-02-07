use bevy::{prelude::*, time::Stopwatch};
use std::{collections::VecDeque, time::Duration};

use crate::buffered_input::GameInput;

/// PlayerState represent state shift of player from game start to end
/// So, PlayerPlugin would control such state transitions of player.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
    Init,
    Idle,
    Shuffled,
    Solving,
    Clear,
    ModeSelectionPopup,
    StatisticsPopup,
    DateSelectionPopup,
    ResponseWaiting,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_playinfo)
            .add_state(PlayerState::Init)
            .add_system_set(
                SystemSet::on_update(PlayerState::Init).with_system(crate::game::try_url_load),
            )
            .add_system_set(SystemSet::on_enter(PlayerState::Idle).with_system(reset_timer))
            .add_system_set(SystemSet::on_enter(PlayerState::Shuffled).with_system(reset_timer))
            .add_system_set(SystemSet::on_enter(PlayerState::Solving).with_system(start_timer))
            .add_system_set(SystemSet::on_update(PlayerState::Solving).with_system(tick_timer))
            .add_system_set(SystemSet::on_enter(PlayerState::Clear).with_system(stop_timer));
    }
}

#[derive(Component, Default)]
pub struct PlayLog {
    play_log: VecDeque<GameInput>,
    redo_buffer: VecDeque<GameInput>,
    pub undo_used: bool,
}

impl PlayLog {
    pub fn add_log(&mut self, input: GameInput) {
        self.play_log.push_back(input);
    }

    pub fn add_redo(&mut self, input: GameInput) {
        self.redo_buffer.push_back(input);
    }

    pub fn undo(&mut self) -> Option<GameInput> {
        self.play_log.pop_back()
    }

    pub fn redo(&mut self) -> Option<GameInput> {
        self.redo_buffer.pop_back()
    }

    pub fn reset(&mut self) {
        self.play_log.clear();
        self.redo_buffer.clear();
        self.undo_used = false;
    }

    pub fn clear_redo_buf(&mut self) {
        self.redo_buffer.clear()
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

    pub fn add_move_count(&mut self) {
        self.move_count += 1;
    }

    pub fn start_timer(&mut self) {
        self.play_timer.unpause();
    }

    pub fn stop_timer(&mut self) {
        self.play_timer.pause();
    }

    pub fn reset(&mut self) {
        self.play_timer.pause();
        self.play_timer.reset();
        self.move_count = 0;
    }

    pub fn get_player_info(&self) -> (Duration, usize) {
        (self.play_timer.elapsed(), self.move_count)
    }
}

fn setup_playinfo(mut commands: Commands) {
    commands
        .spawn(Name::new("PlayerInfo"))
        .insert(PlayerInfo::new());

    commands
        .spawn(Name::new("PlayLog"))
        .insert(PlayLog::default());
}

fn reset_timer(mut player_info: Query<&mut PlayerInfo>) {
    player_info.single_mut().reset();
}

fn start_timer(mut player_info: Query<&mut PlayerInfo>) {
    player_info.single_mut().start_timer();
}

fn tick_timer(time: Res<Time>, mut player_info: Query<&mut PlayerInfo>) {
    player_info.single_mut().play_timer.tick(time.delta());
}

fn stop_timer(mut player_info: Query<&mut PlayerInfo>) {
    player_info.single_mut().stop_timer();
}
