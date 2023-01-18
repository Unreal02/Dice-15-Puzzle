use chrono::{Local, Utc};
use log::info;
use wasm_sockets::{EventClient, Message};

use crate::network::RequestType;

const SERVER_ADDR: &str = "wss://dice15puzzle-server.haje.org"; // actual server

// const SERVER_ADDR: &str = "ws://localhost"; // local server

pub struct Network;

impl Network {
    pub fn request() {
        let mut client = EventClient::new(SERVER_ADDR).unwrap();

        client.set_on_error(Some(Box::new(|error| {
            info!("error {:#?}", error);
        })));

        client.set_on_connection(Some(Box::new(|client: &EventClient| {
            info!("connected {:#?}", client.status);
            info!("Sending message...");
            let date = Utc::now().date_naive();
            let message = serde_json::to_string(&RequestType::GetDailyPuzzle(date)).unwrap();
            client.send_string(&message).unwrap();
        })));

        client.set_on_close(Some(Box::new(|_evt| {
            info!("Connection closed");
        })));

        client.set_on_message(Some(Box::new(|client: &EventClient, message: Message| {
            info!("New Message: {:#?}", message);
        })));
    }
}
