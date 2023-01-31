// shared in client and server

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
/// (position, rotation)
/// index: number written on block (0 means empty)
pub struct BoardString(pub [(u8, u8); 16]);

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    GetDailyPuzzle(NaiveDate),
    GetDailyPuzzleDate,
    GenerateDailyPuzzle(NaiveDate), // used by daily trigger only
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ResponseType {
    GetDailyPuzzle(NaiveDate, BoardString),
    GetDailyPuzzleDate { first: NaiveDate, last: NaiveDate },
    GenerateDailyPuzzle(bool), // used by daily trigger only
}
