// shared in client and server

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, Hash)]
/// (position, rotation)
/// index: number written on block (0 means empty)
pub struct BoardString(pub [(u8, u8); 16]);

#[derive(Copy, Serialize, Deserialize, Clone, Debug)]
pub enum NetworkError {
    KeyAlreadyExist,
    NoEntry,
}

#[allow(dead_code)]
impl BoardString {
    const CORPUS: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#".as_bytes();

    fn into_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        let hash = s.finish();
        hash % (2_u64.pow(36))
    }

    pub fn into_key(&self) -> String {
        let mut key = vec![];
        let mut hash = self.into_hash();
        let mask = 0x3f;

        while hash > 0 {
            let curr = hash & mask;
            key.push(Self::CORPUS[curr as usize] as char);
            hash = hash >> 6;
        }

        while key.len() < 6 {
            key.push('a')
        }

        key.iter().collect()
    }

    pub fn retry_into_key(mut curr: String) -> String {
        let additional = rand::random::<usize>();
        curr.push(Self::CORPUS[additional % 2_usize.pow(6)] as char);
        curr
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    GetDailyPuzzle(NaiveDate),
    GetDailyPuzzleDate,
    GenerateDailyPuzzle(NaiveDate), // used by daily trigger only
    EnrollPuzzleState(String, BoardString),
    GetPuzzleState(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ResponseType {
    GetDailyPuzzle(NaiveDate, BoardString),
    GetDailyPuzzleDate { first: NaiveDate, last: NaiveDate },
    GenerateDailyPuzzle(bool), // used by daily trigger only
    EnrollPuzzleState(Result<(), NetworkError>),
    GetPuzzleState(Result<BoardString, NetworkError>),
}
