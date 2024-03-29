// shared in client and server

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
/// (position, rotation)
/// index: number written on block (0 means empty)
pub struct BoardString(pub Vec<(u8, u8)>);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct DailyRanking {
    pub date: NaiveDate,
    pub time_ranking: Vec<(String, i64)>, //i64 => Duration.to_micros()
    pub move_ranking: Vec<(String, f32)>,
}

#[derive(Copy, Serialize, Deserialize, Clone, Debug)]
pub enum NetworkError {
    KeyAlreadyExist,
    NoEntry,
    NameAlreadyExist,
}

#[allow(dead_code)]
impl BoardString {
    const CORPUS: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#".as_bytes();

    pub fn new(size: usize) -> BoardString {
        Self(vec![(0, 0); size * size])
    }

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

    pub fn to_arr(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for i in self.0.iter() {
            vec.push(i.0);
            vec.push(i.1);
        }
        vec
    }

    pub fn from_arr(query_result: &Vec<u8>) -> Self {
        let size = (query_result.len() as f64).sqrt() as usize;
        let mut board_string = BoardString::new(size);
        for i in 0..size * size {
            board_string.0[i].0 = query_result[i * 2];
            board_string.0[i].1 = query_result[i * 2 + 1];
        }
        board_string
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    GetDailyPuzzle(NaiveDate),
    GetDailyPuzzleDate,
    GenerateDailyPuzzle(NaiveDate), // used by daily trigger only
    EnrollPuzzleState(String, BoardString),
    GetPuzzleState(String),
    EnrollDailyScore(NaiveDate, String, Duration, usize), // zadd 사용해서 처리하면 될듯함
    GetDailyRanking(NaiveDate),
    ClearRanking(NaiveDate),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseType {
    GetDailyPuzzle(NaiveDate, BoardString),
    GetDailyPuzzleDate { first: NaiveDate, last: NaiveDate },
    GenerateDailyPuzzle(bool), // used by daily trigger only
    EnrollPuzzleState(Result<String, NetworkError>),
    GetPuzzleState(Result<BoardString, NetworkError>),
    EnrollDailyScore(Result<(), NetworkError>),
    GetDailyRanking(Result<DailyRanking, NetworkError>),
    ClearRanking(Result<(), NetworkError>),
}
