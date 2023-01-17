use tungstenite::{self, connect, Message};
use url::Url;

const SERVER_ADDR: &str = "wss://dice15puzzle-server.haje.org";

pub struct Network;

impl Network {
    pub fn request() {
        println!("{:?}", Url::parse(SERVER_ADDR).unwrap());
        let (mut socket, _) = connect(Url::parse(SERVER_ADDR).unwrap()).expect("connect fail");

        let request = "get daily puzzle\n";
        socket
            .write_message(Message::Text(request.to_string()))
            .unwrap();

        let response = socket.read_message().unwrap();
        println!("response: {}", response);
    }
}
