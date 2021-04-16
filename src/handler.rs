
use std::collections::HashMap;
// use crossbeam_channel;

use crate::message as msg;

#[derive(Hash,PartialEq,Eq)]
pub enum Outputs {
    Server,
}

pub trait Handler {
    fn handle(&self, m: msg::Message) -> bool;
    fn add_output_channel(&mut self,
        output_type: Outputs,
        channel: crossbeam_channel::Sender<String>);
}

pub struct ServerMessageHandler {
    outputs: HashMap<Outputs, crossbeam_channel::Sender<String>>,
}

impl ServerMessageHandler {
    pub fn new() -> Self {
        ServerMessageHandler{
            outputs: HashMap::new(),
        }
    }

    fn handle_connected(&self, _: msg::Connected) -> bool {
        let register = msg::Register {
            // clientType: self.config.client_type.clone(),
            // game: self.config.game.clone(),
            // name: self.config.name.clone(),
            clientType: "".to_string(),
            game: "".to_string(),
            name: "".to_string(),
        };
        let register_msg = msg::Message::Register(register);
        let response = msg::serialize_message(register_msg);
        if let Err(e) = response {
            println!("Error serialising register message: {}", e);
            return false;
        }
        let response_string = response.unwrap();
        match self.outputs.get(&Outputs::Server) {
            None => false,
            Some(chan) => {
                chan.send(response_string);
                true
            },
        }
    }

    // fn handle_register_success(&self, m: msg::RegisterSuccess) -> bool {
    //     true
    // }

    // fn handle_state(&self, message: msg::State) -> bool {
    //     match message {
    //         msg::State{ r#move: true, .. } => {
    //             let a = msg::Action{
    //                 game: 42,
    //                 key: "test".to_string(),
    //                 action: msg::JsonAction::Null,
    //             };
    //             true
    //         },
    //         _ => false,
    //     }
    // }

    // fn handle_error(&self, _message: msg::Error) -> bool {
    //     true
    // }
}

impl Handler for ServerMessageHandler {
    fn handle(&self, m: msg::Message) -> bool {
        match m {
            // don't pass to client
            msg::Message::Connected(c) => self.handle_connected(c),
            // msg::Message::RegisterSuccess(rs) => self.handle_register_success(rs),
            // // pass to client
            // msg::Message::State(s) => self.handle_state(s),
            // msg::Message::Error(e) => self.handle_error(e),
            _ => false,
        }
    }

    fn add_output_channel(&mut self, output_type: Outputs, channel: crossbeam_channel::Sender<String>){
        self.outputs.insert(output_type, channel);
    }


}

#[cfg(test)]
mod server_message_handler {
    use super::*;
    use std::thread;

    #[test]
    fn handle_server_message_connected() {
        let correct_message = msg::Message::Connected(msg::Connected{});

        let (outbound_server_sender,outbound_server_receiver) = crossbeam_channel::bounded(0);
        let mut handler = ServerMessageHandler::new();
        handler.add_output_channel(Outputs::Server, outbound_server_sender);

        thread::spawn(move || handler.handle(correct_message));

        let response = outbound_server_receiver.recv();
        assert_eq!(
            response.unwrap(),
            r#"{"type":"Register","clientType":"","game":"","name":""}"#.to_string());
    }
}