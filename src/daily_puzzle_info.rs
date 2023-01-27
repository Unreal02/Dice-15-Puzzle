use bevy::prelude::*;
use chrono::NaiveDate;

#[derive(Component, Default, Debug)]
pub struct DailyPuzzleInfo {
    pub first_date: NaiveDate,
    pub last_date: NaiveDate,
    pub current_date: NaiveDate,
}

pub struct DailyPuzzleInfoPlugin;

impl Plugin for DailyPuzzleInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_daily_puzzle_info);
    }
}

fn init_daily_puzzle_info(mut commands: Commands) {
    commands.spawn(DailyPuzzleInfo::default());
}
