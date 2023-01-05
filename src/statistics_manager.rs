use bevy::prelude::*;
use std::time::Duration;

use crate::player::{PlayerInfo, PlayerState};

#[derive(Component, Default)]
pub struct StatisticsManager {
    records: Vec<Duration>,
}

pub struct StatisticsManagerPlugin;

impl Plugin for StatisticsManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_statistics_manager)
            .add_system_set(SystemSet::on_enter(PlayerState::GameClear).with_system(on_game_clear));
    }
}

fn spawn_statistics_manager(mut commands: Commands) {
    commands.spawn(StatisticsManager::default());
}

fn on_game_clear(
    mut statistics_manager_query: Query<&mut StatisticsManager>,
    player_info_query: Query<&PlayerInfo>,
) {
    let mut statistics_manager = statistics_manager_query.single_mut();
    let (time, _) = player_info_query.single().get_player_info();
    statistics_manager.records.push(time);
    println!("game clear");
    println!("  number : {:?}", statistics_manager.records.len());
    println!(
        "  average: {:?}",
        statistics_manager.records.iter().sum::<Duration>()
            / statistics_manager.records.len() as u32
    );
    println!(
        "  best   : {:?}",
        statistics_manager.records.iter().min().unwrap()
    );
    println!(
        "  worst  : {:?}",
        statistics_manager.records.iter().max().unwrap()
    );
    println!("  details:");
    for t in statistics_manager.records.iter() {
        println!("    {:?}", t);
    }
    println!();
}
