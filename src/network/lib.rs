use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    GetDailyPuzzle(NaiveDate),
}
