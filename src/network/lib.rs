// shared in client and server

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
/// (position, rotation)
/// index: number written on block (0 means empty)
pub struct BoardString(pub [(u8, u8); 16]);

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    GetDailyPuzzle(NaiveDate),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    GetDailyPuzzle(BoardString),
}
