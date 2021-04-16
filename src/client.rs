use crate::handler;
use crate::message as msg;

// type RawMessage = str;
// type MessageHandler = fn(&RawMessage);

// pub trait Connection {
//     fn read(&self) -> &str;
//     fn write(&self, msg: &str);
// }

// #[derive(Clone)]
pub struct ClientConfig {
    pub client_type: String,
    pub game: String,
    pub name: String,
}

pub struct Client {
    handler: Box<handler::Handler>,
}

impl Client {
    fn new(handler: Box<handler::Handler>) -> Self {
        Client{
            handler: handler
        }
    }

    // fn pipe(&self, raw_message: &str) -> Option<bool>{
    //     let message = self.parse_raw_message(raw_message)?;
    //     let response = self.handle_message(message);
    //     Some(response)
    // }

    fn parse_raw_message(&self, raw_message: &str) -> Option<msg::Message> {
        let message_result = msg::deserialize_message(raw_message);
        if let Err(_) = message_result {
            return None;
        };
        let message = message_result.unwrap();
        Some(message)
    }

    fn handle_message(&self, message: msg::Message) -> bool {
        self.handler.handle(message)
    }
}

#[cfg(test)]
mod client {
    use super::*;
    use std::thread;

    fn new_test_client() -> Client {
        let handler = handler::ServerMessageHandler::new();
        Client::new(Box::new(handler))
    }

	#[test]
    fn parsing_correct_connected_message_returns_Some() {
        let client = new_test_client();

        let incoming_message = r#"{"type": "Connected"}"#;
        let result = client.parse_raw_message(incoming_message);

        let ok = match result {
            Some(msg::Message::Connected(_)) => true,
            _ => false,
        };
        assert_eq!(ok, true);
    }

    #[test]
    fn parsing_incorrect_message_returns_None() {
        let client = new_test_client();

        let incoming_message = r#"{"This is": "nonsense"}"#;
        let result = client.parse_raw_message(incoming_message);
        assert_eq!(result.is_none(), true);
    }
}