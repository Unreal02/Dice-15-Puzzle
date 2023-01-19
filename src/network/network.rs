use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use chrono::NaiveDate;

use crate::network::RequestType;

const SERVER_ADDR: &str = "https://dice15puzzle-server.haje.org"; // actual server

// const SERVER_ADDR: &str = "http://localhost:1515"; // local server

pub struct Network;

impl Network {
    fn request(req: RequestType) {
        let thread_pool = AsyncComputeTaskPool::get();
        thread_pool.spawn(async move {
            let res = reqwest::get(SERVER_ADDR)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            info!("{}", res);
        });
        info!("asdf");
    }

    pub fn get_daily_puzzle(date: NaiveDate) {
        Self::request(RequestType::GetDailyPuzzle(date));
        info!("qwerqwer");
    }
}
