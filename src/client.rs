use std::collections::HashMap;

type RawMessage = String;

#[derive(Debug, PartialEq, Eq, Hash)]
enum MessageType {
    Action,
    Error,
    Register,
    Start,
    State,
    Stop,
    EngineAction,
    EngineState,
}

type MessageHandler = fn(RawMessage);

pub struct Client {
    // started: bool,
    // connection: i32,  // websocket connection
    handlers: HashMap<MessageType, MessageHandler>,
}

pub fn new_client() -> Client {
    Client {
        handlers: HashMap::new(),
    }
}

impl Client {
    fn add_handler(&mut self, name: MessageType, func: MessageHandler) {
        self.handlers.insert(name, func);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dummy(_: String) {}

	#[test]
    fn test_add_handler() {
		let mut c = new_client();
        c.add_handler(MessageType::Action, dummy);
        assert_eq!(true, c.handlers.contains_key(&MessageType::Action));

        c.handlers[&MessageType::Action]("Test".to_string());
    }
}