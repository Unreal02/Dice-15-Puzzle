use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    local_storage::LocalStorage,
    player::{PlayerInfo, PlayerState},
    ui::GameMode,
    utils::duration_to_string,
};

#[derive(Serialize, Deserialize, Component)]
pub struct StatisticsManager {
    mode: GameMode,
    time_records: Vec<Duration>,
    move_records: Vec<usize>,
}

impl Default for StatisticsManager {
    fn default() -> Self {
        Self {
            mode: GameMode::Practice,
            time_records: vec![],
            move_records: vec![],
        }
    }
}

impl StatisticsManager {
    pub fn set_mode(&mut self, mode: GameMode) {
        self.mode = mode;
    }

    fn save_storage(&self) {
        LocalStorage::set_statistics(self);
    }

    pub fn push(&mut self, info: &PlayerInfo) {
        let (time, move_count) = info.get_player_info();
        match self.mode {
            GameMode::TimeAttack => self.time_records.push(time),
            GameMode::MinimalMovement => self.move_records.push(move_count),
            _ => unreachable!(),
        }
        self.save_storage();
    }

    pub fn delete_statistics(&mut self) {
        match self.mode {
            GameMode::TimeAttack => self.time_records.clear(),
            GameMode::MinimalMovement => self.move_records.clear(),
            _ => unreachable!(),
        }
        self.save_storage();
    }

    pub fn get_record(&self, i: usize) -> String {
        match self.mode {
            GameMode::TimeAttack => duration_to_string(self.time_records[i]),
            GameMode::MinimalMovement => format!("{}", self.move_records[i]),
            _ => unreachable!(),
        }
    }

    pub fn solves(&self) -> usize {
        match self.mode {
            GameMode::TimeAttack => self.time_records.len(),
            GameMode::MinimalMovement => self.move_records.len(),
            _ => unreachable!(),
        }
    }

    pub fn average(&self) -> String {
        match self.mode {
            GameMode::TimeAttack => duration_to_string(
                self.time_records.iter().sum::<Duration>() / self.solves() as u32,
            ),
            GameMode::MinimalMovement => {
                format!(
                    "{:.2}",
                    self.move_records.iter().sum::<usize>() as f32 / self.solves() as f32
                )
            }
            _ => unreachable!(),
        }
    }

    pub fn best(&self) -> String {
        match self.mode {
            GameMode::TimeAttack => duration_to_string(*self.time_records.iter().min().unwrap()),
            GameMode::MinimalMovement => format!("{}", self.move_records.iter().min().unwrap()),
            _ => unreachable!(),
        }
    }

    pub fn worst(&self) -> String {
        match self.mode {
            GameMode::TimeAttack => duration_to_string(*self.time_records.iter().max().unwrap()),
            GameMode::MinimalMovement => format!("{}", self.move_records.iter().max().unwrap()),
            _ => unreachable!(),
        }
    }

    pub fn export(&self) {
        let mut export_string = format!(
            "Dice 15 Puzzle (dice15puzzle.haje.org)\nStatistics\n\nSolves: {}",
            self.solves()
        );
        if self.solves() > 0 {
            export_string.push_str(&format!(
                "\nAverage: {}\nBest: {}\nWorst: {}\nDetails:\n",
                self.average(),
                self.best(),
                self.worst()
            ));
            for i in 0..self.solves() {
                export_string.push_str(&format!("{}. {}\n", i + 1, self.get_record(i)));
            }
        }
        info!("{}", export_string);
        let clipboard = web_sys::window().unwrap().navigator().clipboard().unwrap();
        let _ = clipboard.write_text(&export_string);
    }
}

pub struct StatisticsManagerPlugin;

impl Plugin for StatisticsManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_statistics_manager)
            .add_system_set(SystemSet::on_enter(PlayerState::Clear).with_system(on_game_clear))
            .add_system(set_game_mode);
    }
}

fn spawn_statistics_manager(mut commands: Commands) {
    commands.spawn(LocalStorage::get_statistics().unwrap_or_default());
}

fn on_game_clear(
    mut statistics_manager_query: Query<&mut StatisticsManager>,
    player_info_query: Query<&PlayerInfo>,
    game_mode: Res<State<GameMode>>,
) {
    let mut statistics_manager = statistics_manager_query.single_mut();

    if *game_mode.current() != GameMode::TimeAttack
        && *game_mode.current() != GameMode::MinimalMovement
    {
        return;
    }

    statistics_manager.push(player_info_query.single());
}

fn set_game_mode(
    mut statistics_manager_query: Query<&mut StatisticsManager>,
    game_mode: Res<State<GameMode>>,
) {
    if game_mode.is_changed() {
        statistics_manager_query
            .single_mut()
            .set_mode(*game_mode.current());
    }
}
