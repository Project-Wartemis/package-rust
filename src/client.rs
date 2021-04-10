use crate::message as msg;
// use crate::message::MessageTrait;
use crossbeam_channel;

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
    config: ClientConfig,
    // connection: Box<dyn Connection>,
    // default_handler: MessageHandler,
    // started: bool,
}

impl Client {
    fn new(config: ClientConfig) -> Self {
        Client{
            config: config,
        }
    }

    fn pipe(&self, raw_message: &str) -> Option<String>{
        let message = self.parse_raw_message(raw_message)?;
        let response = self.handle_message(message)?;
        match msg::serialize_message(response) {
            Ok(r) => Some(r),
            _ => None,
        }
    }

    fn parse_raw_message(&self, raw_message: &str) -> Option<msg::Message> {
        let message_result = msg::deserialize_message(raw_message);
        if let Err(_) = message_result {
            return None;
        };
        let message = message_result.unwrap();
        Some(message)
    }

    fn handle_message(&self, message: msg::Message) -> Option<msg::Message> {
        match message {
            // from server - don't pass on to client
            msg::Message::Connected(c) => self.handle_connected(c),
            msg::Message::RegisterSuccess(rs) => self.handle_register_success(rs),
            // from server - pass to client
            msg::Message::State(s) => self.handle_state(s),
            msg::Message::Error(e) => self.handle_error(e),
            // from client - pass to server
            msg::Message::Action(s) => None,
            // ignore others
            _ => None,
        }
    }

    fn handle_connected(&self, _: msg::Connected) -> Option<msg::Message> {
        let register = msg::Register {
            clientType: self.config.client_type.clone(),
            game: self.config.game.clone(),
            name: self.config.name.clone(),
        };
        Some(msg::Message::Register(register))
    }

    fn handle_register_success(&self, _: msg::RegisterSuccess) -> Option<msg::Message> {
        None
    }

    fn handle_state(&self, message: msg::State) -> Option<msg::Message> {
        match message {
            msg::State{ r#move: true, .. } => {
                let a = msg::Action{
                    game: 42,
                    key: "test".to_string(),
                    action: msg::JsonAction::Null,
                };
                Some(msg::Message::Action(a))
            },
            _ => None,
        }
    }

    fn handle_error(&self, _message: msg::Error) -> Option<msg::Message> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn new_test_client() -> Client {
        let client_config = ClientConfig{
            client_type: "test-clientType".to_string(),
            game: "test-game".to_string(),
            name: "test-name".to_string(),
        };
        Client::new(client_config)
    }

	#[test]
    fn receiving_a_correct_raw_message_returns_some() {
        let client = new_test_client();

        let incoming_message = r#"{"type": "Connected"}"#;
        let result = client.parse_raw_message(incoming_message);
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn receiving_an_invalid_raw_message_returns_none() {
        let client = new_test_client();

        let incoming_message = r#"{"This is": "nonsense"}"#;
        let result = client.parse_raw_message(incoming_message);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn message_type_state_triggers_response_register() {
        let client = new_test_client();
        let incoming_message = msg::Message::Connected(msg::Connected{});
        let result = client.handle_message(incoming_message);

        assert_eq!(result.is_some(), true);

        match result.unwrap() {
            msg::Message::Register(r) => {
                assert_eq!(r.clientType, client.config.client_type);
                assert_eq!(r.game, client.config.game);
                assert_eq!(r.name, client.config.name);
            },
            _ => (),
        };
    }

    #[test]
    fn incomming_message_type_register_success_triggers_no_response() {
        let client = new_test_client();

        let incoming_message = msg::Message::RegisterSuccess(msg::RegisterSuccess{id: 1});
        let result = client.handle_message(incoming_message);

        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn message_type_state_triggers_no_response() {
        let client = new_test_client();

        let incoming_message = msg::Message::State(msg::State{
            game: 42,
            key: "dummy_key".to_string(),
            turn: 0,
            r#move: true,
            state: msg::JsonState::Null,
        });
        let result = client.handle_message(incoming_message);

        assert_eq!(result.is_some(), true)
    }

    #[test]
    fn message_type_error_triggers_no_response() {
        let client = new_test_client();

        let incoming_message = msg::Message::Error(msg::Error{
            message: "error message".to_string(),
        });
        let result = client.handle_message(incoming_message);

        assert_eq!(result.is_none(), true)
    }


    #[test]
    fn test_pipe() {
        let correct_message = r#"{"type": "Connected"}"#;
        let expected_response = r#"{"type":"Register","clientType":"test-clientType","game":"test-game","name":"test-name"}"#;
        let client = new_test_client();

        let response = client.pipe(correct_message);
        assert_eq!(response.unwrap(), expected_response.to_string());
    }

    #[test]
    fn chan() {
        // let connected = r#"{"type": "Connected"}"#;

        // let (s,r) = crossbeam_channel::bounded(0);
        // thread::spawn(move || s.send(connected).unwrap());

        // // Receiving blocks until a send operation appears on the other side.
        // assert_eq!(r.recv(), Ok(connected));
    }

}