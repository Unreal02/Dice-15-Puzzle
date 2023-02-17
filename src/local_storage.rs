use crate::{daily_puzzle_info::ClearHistory, statistics_manager::StatisticsManager};

const INPUT_INVERSION: &str = "input_inversion";
const MOVE_IMMEDIATE: &str = "move_immediate";
const DAILY_PUZZLE_CLEAR_HISTORY: &str = "daily_puzzle_clear_history";
const STATISTICS: &str = "statistics";

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
        local_storage.set_item(&key, &value).unwrap();
    }

    pub fn get_input_inversion() -> Option<bool> {
        match Self::get(INPUT_INVERSION) {
            Some(value) => Some(serde_json::from_str(&value).unwrap()),
            None => None,
        }
    }

    pub fn set_input_inversion(value: &bool) {
        Self::set(INPUT_INVERSION, &serde_json::to_string(value).unwrap());
    }

    pub fn get_move_immediate() -> Option<bool> {
        match Self::get(MOVE_IMMEDIATE) {
            Some(value) => Some(serde_json::from_str(&value).unwrap()),
            None => None,
        }
    }

    pub fn set_move_immediate(value: &bool) {
        Self::set(MOVE_IMMEDIATE, &serde_json::to_string(value).unwrap());
    }

    pub fn get_daily_puzzle_clear_history() -> Option<ClearHistory> {
        match Self::get(DAILY_PUZZLE_CLEAR_HISTORY) {
            Some(value) => Some(serde_json::from_str(&value).unwrap()),
            None => None,
        }
    }

    pub fn set_daily_puzzle_clear_history(value: &ClearHistory) {
        Self::set(
            DAILY_PUZZLE_CLEAR_HISTORY,
            &serde_json::to_string(value).unwrap(),
        );
    }

    pub fn get_statistics() -> Option<StatisticsManager> {
        match Self::get(STATISTICS) {
            Some(value) => Some(serde_json::from_str(&value).unwrap()),
            None => None,
        }
    }

    pub fn set_statistics(value: &StatisticsManager) {
        Self::set(STATISTICS, &serde_json::to_string(value).unwrap());
    }
}
