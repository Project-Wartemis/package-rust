use std::collections::HashMap;
use crate::message::{Message,*};

type RawMessage = String;
type MessageHandler = fn(&RawMessage);

pub trait Connection {
    fn read(&self) -> String;
    fn write(&self, msg: &String);
}

pub struct Client {
    connection: Box<dyn Connection>,
    started: bool,
}

pub fn new_client(connection: Box<dyn Connection>) -> Client {
    Client {
        started: false,
        connection: connection,
    }
}

impl Client {
    fn default_handler(&self, msg: &RawMessage) {

    }

    fn handle_raw_message(&self, msg: RawMessage) {
        let result = deserialize_message(&msg);
        if let Err(_) = result {
            self.default_handler(&msg);
            return;
        }
        let msg_struct = result.unwrap();

        match msg_struct {
            Message::Connected(m) => self.handle_connected(m),
            Message::RegisterSuccess(m) => self.handle_register_success(m),
            _ => self.default_handler(&msg),
        }
    }

    fn send_msg(&self, json: &String) {
        self.connection.write(json)
    }

    fn handle_connected(&self, msg: Connected) {
        let reg = Register{
            clientType: "".to_string(),
            game: "Game".to_string(),
            name: "name".to_string(),
        };
        let json = reg.to_json();
        match json {
            Ok(json) => self.send_msg(&json),
            Err(_) => println!("Failed to send message"),
        }
    }

    fn handle_register_success(&self, _: RegisterSuccess) {
        println!("Registered!");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_handler(_: &String) {}

    struct DummyConnection {}

    impl Connection for DummyConnection {
        fn read(&self) -> RawMessage {
            "ok".to_string()
        }
        fn write(&self, msg: &RawMessage) {
            assert_eq!("ok",msg);
        }
    }

    fn new_dummy_connection() -> Box<DummyConnection> {
        Box::new(DummyConnection{})
    }


	#[test]
    fn handle_raw_message() {
        let dummy_connection: Box<dyn Connection> = new_dummy_connection();
		let c = new_client(dummy_connection);

        let msg: RawMessage = "ok".to_string();
        c.handle_raw_message(msg)
    }
}