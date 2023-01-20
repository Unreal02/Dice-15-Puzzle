use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use chrono::NaiveDate;

use crate::network::*;

const SERVER_ADDR: &str = "https://dice15puzzle-server.haje.org"; // actual server

// const SERVER_ADDR: &str = "http://localhost:1515"; // local server

pub struct Network;

impl Network {
    fn request(req: RequestType) {
        let thread_pool = AsyncComputeTaskPool::get();
        thread_pool.spawn(async move {
            let client = reqwest::Client::new();
            let res = client
                .post(SERVER_ADDR)
                .body(serde_json::to_string(&req).unwrap())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            info!("raw text: {}", res);
            if let Ok(response_type) = serde_json::from_str(&res) as Result<ResponseType, _> {
                info!("{:?}", response_type);
            }
        });
        info!("send request");
    }

    pub fn get_daily_puzzle(date: NaiveDate) {
        Self::request(RequestType::GetDailyPuzzle(date));
        info!("get_daily_puzzle");
    }
}
