use bevy::{prelude::*, utils::HashMap};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{
    game::GameState,
    network::{BoardString, Network, NetworkChannel},
    player::PlayerState,
    ui::GameMode,
    utils::string_to_board,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ClearHistory(HashMap<i32, HashMap<u32, Vec<bool>>>);

#[derive(Component, Default, Debug)]
pub struct DailyPuzzleInfo {
    pub first_date: NaiveDate,
    pub last_date: NaiveDate,
    pub current_date: NaiveDate,
    daily_puzzles: HashMap<NaiveDate, BoardString>,
    pub clear_history: ClearHistory,
}

impl ClearHistory {
    pub fn get(&self, date: NaiveDate) -> bool {
        let year = date.year_ce().1 as i32;
        let month = date.month();
        let day = date.day();
        if let Some(year_history) = self.0.get(&year) {
            if let Some(month_history) = year_history.get(&month) {
                month_history[day as usize - 1]
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set(&mut self, date: NaiveDate, val: bool) {
        let year = date.year_ce().1 as i32;
        let month = date.month();
        let day = date.day();
        if !self.0.contains_key(&year) {
            self.0.insert(year, HashMap::new());
        }
        let year_history = self.0.get_mut(&year).unwrap();
        let days_in_month = |y: i32, m: u32| {
            NaiveDate::from_ymd_opt(
                match m {
                    12 => y + 1,
                    _ => y,
                },
                match m {
                    12 => 1,
                    _ => m + 1,
                },
                1,
            )
            .unwrap()
            .signed_duration_since(NaiveDate::from_ymd_opt(y, m, 1).unwrap())
            .num_days()
        };
        if !year_history.contains_key(&month) {
            year_history.insert(month, vec![false; days_in_month(year, month) as usize]);
        }
        let month_history = year_history.get_mut(&month).unwrap();
        month_history[day as usize - 1] = val;
    }
}

impl DailyPuzzleInfo {
    pub fn insert_daily_puzzle(&mut self, date: NaiveDate, board_string: BoardString) {
        assert!(!self.daily_puzzles.contains_key(&date));
        self.daily_puzzles.insert(date, board_string);
    }

    pub fn load_daily_puzzle(
        &self,
        date: NaiveDate,
        transforms: &mut Query<&mut Transform>,
        game: &mut GameState,
        player_state: &mut ResMut<State<PlayerState>>,
        network_channel: &Res<NetworkChannel>,
    ) -> bool {
        if let Some(&board_string) = self.daily_puzzles.get(&date) {
            string_to_board(board_string, transforms, game);
            if *player_state.current() != PlayerState::Shuffled {
                // inactive stack에 있는 것이 무엇이든 Shuffled로 바꾸기 위해 replace 사용
                player_state.replace(PlayerState::Shuffled).unwrap();
            }
            true
        } else {
            Network::get_daily_puzzle(date, player_state, network_channel);
            false
        }
    }
}

pub struct DailyPuzzleInfoPlugin;

impl Plugin for DailyPuzzleInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_daily_puzzle_info)
            .add_system_set(
                SystemSet::on_enter(PlayerState::Clear).with_system(on_clear_daily_puzzle),
            );
    }
}

fn init_daily_puzzle_info(mut commands: Commands) {
    let mut daily_puzzle_info = DailyPuzzleInfo::default();

    // clear history
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Some(clear_history) = local_storage
        .get_item("daily_puzzle_clear_history")
        .unwrap()
    {
        daily_puzzle_info.clear_history = serde_json::from_str(&clear_history).unwrap();
    }

    commands.spawn(daily_puzzle_info);
}

fn on_clear_daily_puzzle(
    game_mode: Res<State<GameMode>>,
    mut daily_puzzle_info_query: Query<&mut DailyPuzzleInfo>,
) {
    if *game_mode.current() != GameMode::DailyPuzzle {
        return;
    }
    let mut daily_puzzle_info = daily_puzzle_info_query.single_mut();

    let date = daily_puzzle_info.current_date;
    daily_puzzle_info.clear_history.set(date, true);

    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(
            "daily_puzzle_clear_history",
            &serde_json::to_string(&daily_puzzle_info.clear_history).unwrap(),
        )
        .unwrap();
}
