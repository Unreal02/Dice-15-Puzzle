use bevy::{prelude::*, utils::HashMap};
use chrono::NaiveDate;

use crate::{
    game::GameState,
    network::{BoardString, Network, NetworkChannel},
    player::PlayerState,
    utils::string_to_board,
};

#[derive(Component, Default, Debug)]
pub struct DailyPuzzleInfo {
    pub first_date: NaiveDate,
    pub last_date: NaiveDate,
    pub current_date: NaiveDate,
    daily_puzzles: HashMap<NaiveDate, BoardString>,
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
        app.add_startup_system(init_daily_puzzle_info);
    }
}

fn init_daily_puzzle_info(mut commands: Commands) {
    commands.spawn(DailyPuzzleInfo::default());
}
