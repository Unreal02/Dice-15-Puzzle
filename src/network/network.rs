use bevy::prelude::*;
use chrono::NaiveDate;
use wasm_sockets::{EventClient, Message};

use crate::network::RequestType;

const SERVER_ADDR: &str = "wss://dice15puzzle-server.haje.org"; // actual server

// const SERVER_ADDR: &str = "ws://localhost"; // local server

pub struct Network;

impl Network {
    fn request(req: RequestType) {
        let mut client = EventClient::new(SERVER_ADDR).unwrap();

        client.set_on_error(Some(Box::new(|error| {
            info!("Error\n{:#?}", error);
        })));

        client.set_on_connection(Some(Box::new(move |client: &EventClient| {
            info!("Connected\n{:#?}", client.status);
            info!("Sending message...");
            let message = serde_json::to_string(&req).unwrap();
            client.send_string(&message).unwrap();
        })));

        client.set_on_close(Some(Box::new(|_evt| {
            info!("Connection closed");
        })));

        client.set_on_message(Some(Box::new(|client: &EventClient, message: Message| {
            info!("New Message: {:#?}", message);
        })));
    }

    pub fn get_daily_puzzle(date: NaiveDate) {
        Self::request(RequestType::GetDailyPuzzle(date));
    }
}
