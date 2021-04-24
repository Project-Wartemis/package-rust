use crate::handler;
use crate::message as msg;
use crossbeam_channel::{select,unbounded};
use futures::executor::block_on;

use std::thread;



pub struct Client {
    handler: Box<dyn handler::Handler>,
    inc_server_chan: crossbeam_channel::Receiver<String>,
    inc_bot_chan: crossbeam_channel::Receiver<String>,
}

impl Client {
    fn new(
        handler: Box<dyn handler::Handler>,
        inc_server_chan: crossbeam_channel::Receiver<String>,
        inc_bot_chan: crossbeam_channel::Receiver<String>) -> Self {
        Client{
            handler: handler,
            inc_server_chan: inc_server_chan,
            inc_bot_chan: inc_bot_chan,
        }
    }

    fn start(&self) -> Result<(),crossbeam_channel::RecvError>{
        loop {
            select!{
                recv(self.inc_server_chan) -> msg => self.handle(msg)?,
                recv(self.inc_bot_chan) -> msg => self.handle(msg)?,
            };
        }
    }

    fn handle(&self, channel_output: Result<String, crossbeam_channel::RecvError>) -> Result<(), crossbeam_channel::RecvError> {
        let message_string = channel_output?;
        let message = msg::deserialize_message(&message_string).unwrap();
        let response = self.handler.handle(message_string,message);
        Ok(())
    }
}

#[cfg(test)]
mod client {
    use super::*;
    use std::thread;
    use crate::handler::*;

    fn default_client_config() -> handler::ClientConfig {
        handler::ClientConfig{
            clientType: "bot".to_string(),
            game: "test_game".to_string(),
            name: "test_bot".to_string(),
        }
    }

	#[test]
    fn received_connected_message_should_respond_with_register_message() {
        let output_type = handler::Outputs::Server;
        let msg_to_send = r#"{"type": "Connected"}"#;
        let expected_reply = r#"{"type":"Register","clientType":"bot","game":"test_game","name":"test_bot"}"#;

        doit(output_type, msg_to_send, expected_reply);
    }

    #[test]
    fn receive_state_message_and_pass_on_to_bot() {
        let output_type = handler::Outputs::Bot;
        let msg_to_send = r#"{"type": "State", "other": "fields"}"#;
        let expected_reply = r#"{"type": "State", "other": "fields"}"#;

        doit(output_type, msg_to_send, expected_reply);
    }

    #[test]
    fn receive_error_message_and_pass_on_to_bot() {
        let output_type = handler::Outputs::Bot;
        let msg_to_send = r#"{"type": "Error", "message": "string"}"#;
        let expected_reply = r#"{"type": "Error", "message": "string"}"#;

        doit(output_type, msg_to_send, expected_reply);
    }

    #[test]
    fn receive_action_message_and_pass_on_to_server() {
        let output_type = handler::Outputs::Server;
        let msg_to_send = r#"{"type": "Action", "message": "string"}"#;
        let expected_reply = r#"{"type": "Action", "message": "string"}"#;

        doit(output_type, msg_to_send, expected_reply)
    }

    fn doit(output_type: handler::Outputs, msg_to_send: &str, expected_reply: &str) {
        let (svr_inc_snd, svr_inc_rec) = crossbeam_channel::bounded(1);
        let (bot_inc_snd, bot_inc_rec) = crossbeam_channel::bounded(1);

        let (output_snd, output_rec) = crossbeam_channel::bounded(1);

        let output_type_clone = output_type.clone();

        // Create and start client
        thread::spawn(move || {
            let mut handler = handler::MessageHandler::new(default_client_config());
            handler.add_output_channel(output_type_clone, output_snd);

            let c = Client::new(Box::new(handler), svr_inc_rec, bot_inc_rec);

            let r = c.start();
            if let Err(e) = r {
                assert_eq!(format!("{}", e), "");
            }
        });

        // client receives Connected message from Server
        let input_msg = msg_to_send.to_string();
        match output_type {
            handler::Outputs::Server => svr_inc_snd.send(input_msg),
            handler::Outputs::Bot => bot_inc_snd.send(input_msg),
        };

        // get reply
        let response = output_rec.recv().unwrap();
        assert_eq!(response, expected_reply.to_string())
    }
}