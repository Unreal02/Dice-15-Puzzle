use log::info;
use wasm_sockets::{EventClient, Message};

const SERVER_ADDR: &str = "wss://dice15puzzle-server.haje.org";

pub struct Network;

impl Network {
    pub fn request() {
        let request = "get daily puzzle\n";

        let mut client = EventClient::new(SERVER_ADDR).unwrap();

        client.set_on_error(Some(Box::new(|error| {
            info!("error {:#?}", error);
        })));

        client.set_on_connection(Some(Box::new(|client: &EventClient| {
            info!("connected {:#?}", client.status);
            info!("Sending message...");
            client.send_string(request).unwrap();
            client.send_binary(vec![20]).unwrap();
        })));

        client.set_on_close(Some(Box::new(|_evt| {
            info!("Connection closed");
        })));

        client.set_on_message(Some(Box::new(|client: &EventClient, message: Message| {
            info!("New Message: {:#?}", message);
        })));
    }
}
