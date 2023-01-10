use bevy::prelude::*;
use std::time::Duration;

use crate::{
    player::{PlayerInfo, PlayerState},
    ui::GameMode,
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
