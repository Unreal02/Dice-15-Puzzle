use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::time::Duration;

use crate::{
    player::{PlayerInfo, PlayerState},
    ui::GameMode,
    utils::duration_to_string,
};

#[derive(Component, Default)]
pub struct StatisticsManager {
    pub records: Vec<Duration>,
}

impl StatisticsManager {
    pub fn solves(&self) -> usize {
        self.records.len()
    }

    pub fn average(&self) -> Duration {
        self.records.iter().sum::<Duration>() / self.records.len() as u32
    }

    pub fn best(&self) -> Duration {
        *self.records.iter().min().unwrap()
    }

    pub fn worst(&self) -> Duration {
        *self.records.iter().max().unwrap()
    }

    pub fn export(&self) {
        let mut export_string = format!(
            "Dice 15 Puzzle (dice15puzzle.haje.org)\nStatistics\n\nSolves: {}",
            self.solves()
        );
        if self.solves() > 0 {
            export_string.push_str(&format!(
                "\nAverage: {}\nBest: {}\nWorst: {}\nDetails:\n",
                duration_to_string(self.average()),
                duration_to_string(self.best()),
                duration_to_string(self.worst())
            ));
            for (i, &duration) in self.records.iter().enumerate() {
                export_string.push_str(&format!("{}. {}\n", i + 1, duration_to_string(duration)));
            }
        }
        info!("{}", export_string);
        let mut clipboard = ClipboardContext::new().unwrap();
        clipboard.set_contents(export_string).unwrap();
    }
}

pub struct StatisticsManagerPlugin;

impl Plugin for StatisticsManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_statistics_manager)
            .add_system_set(SystemSet::on_enter(PlayerState::Clear).with_system(on_game_clear));
    }
}

fn spawn_statistics_manager(mut commands: Commands) {
    commands.spawn(StatisticsManager::default());
}

fn on_game_clear(
    mut statistics_manager_query: Query<&mut StatisticsManager>,
    player_info_query: Query<&PlayerInfo>,
    game_mode: Res<State<GameMode>>,
) {
    if *game_mode.current() != GameMode::TimeAttack {
        return;
    }

    let mut statistics_manager = statistics_manager_query.single_mut();
    let (time, _) = player_info_query.single().get_player_info();
    statistics_manager.records.push(time);
}
