use crate::{
    daily_puzzle_info::ClearHistory,
    game::{BoardSize, Difficulty},
    statistics_manager::StatisticsManager,
    ui::SkipHowToPlay,
};

const INPUT_INVERSION: &str = "input_inversion";
const MOVE_IMMEDIATE: &str = "move_immediate";
const DAILY_PUZZLE_CLEAR_HISTORY: &str = "daily_puzzle_clear_history";
const STATISTICS: &str = "statistics";
const SKIP_HOW_TO_PLAY: &str = "skip_how_to_play";
const BOARD_SIZE: &str = "board_size";
const DIFFICULTY: &str = "difficulty";

pub struct LocalStorage;

impl LocalStorage {
    fn get(key: &str) -> Option<String> {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        match local_storage.get_item(key) {
            Ok(value) => value,
            Err(_) => todo!(),
        }
    }

    fn set(key: &str, value: &str) {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        local_storage.set_item(key, value).unwrap();
    }

    pub fn get_input_inversion() -> Option<bool> {
        Self::get(INPUT_INVERSION).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_input_inversion(value: &bool) {
        Self::set(INPUT_INVERSION, &serde_json::to_string(value).unwrap());
    }

    pub fn get_move_immediate() -> Option<bool> {
        Self::get(MOVE_IMMEDIATE).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_move_immediate(value: &bool) {
        Self::set(MOVE_IMMEDIATE, &serde_json::to_string(value).unwrap());
    }

    pub fn get_daily_puzzle_clear_history() -> Option<ClearHistory> {
        Self::get(DAILY_PUZZLE_CLEAR_HISTORY).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_daily_puzzle_clear_history(value: &ClearHistory) {
        Self::set(
            DAILY_PUZZLE_CLEAR_HISTORY,
            &serde_json::to_string(value).unwrap(),
        );
    }

    pub fn get_statistics() -> Option<StatisticsManager> {
        Self::get(STATISTICS).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_statistics(value: &StatisticsManager) {
        Self::set(STATISTICS, &serde_json::to_string(value).unwrap());
    }

    pub fn get_skip_how_to_play() -> Option<SkipHowToPlay> {
        Self::get(SKIP_HOW_TO_PLAY).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_skip_how_to_play(value: &SkipHowToPlay) {
        Self::set(SKIP_HOW_TO_PLAY, &serde_json::to_string(value).unwrap());
    }

    pub fn get_board_size() -> Option<BoardSize> {
        Self::get(BOARD_SIZE).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_board_size(value: &BoardSize) {
        Self::set(BOARD_SIZE, &serde_json::to_string(value).unwrap());
    }

    pub fn get_difficulty() -> Option<Difficulty> {
        Self::get(DIFFICULTY).map(|value| serde_json::from_str(&value).unwrap())
    }

    pub fn set_difficulty(value: &Difficulty) {
        Self::set(DIFFICULTY, &serde_json::to_string(value).unwrap());
    }
}
