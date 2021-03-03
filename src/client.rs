use std::collections::HashMap;


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

type RawMessage = String;
type MessageHandler = fn(&RawMessage);

trait Connection {
    fn read(&self) -> RawMessage;
    fn write(&self, msg: &RawMessage);
}

pub struct Client {
    started: bool,
    connection: Box<dyn Connection>,  // websocket connection
    handlers: HashMap<MessageType, MessageHandler>,
    defaultHandler: MessageHandler,
}

pub fn new_client(
        handlers: HashMap<MessageType, MessageHandler>,
        defaultHandler: MessageHandler,
        connection: Box<dyn Connection>) -> Client {
    Client {
        started: false,
        connection: connection,
        handlers: handlers,
        defaultHandler: defaultHandler,
    }
}

fn parse_raw_message(msg: &RawMessage) -> &MessageType {
    &MessageType::Action
}

impl Client {
    fn handle_raw_message(&self, msg: &RawMessage) {
        let msgType = parse_raw_message(msg);

        let handler: MessageHandler = self.defaultHandler;
        if self.handlers.contains_key(msgType) {
            handler = self.handlers[msgType];
        }

        handler(msg);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_handler(_: &String) {}

	#[test]
    fn test_add_handler() {
        let handlers: HashMap<MessageType, MessageHandler> = HashMap::new();
		let c = new_client(handlers, dummy_handler);
        // assert_eq!(true, c.handlers.contains_key(&MessageType::Action));

        // c.handlers[&MessageType::Action]("Test".to_string());
    }
}