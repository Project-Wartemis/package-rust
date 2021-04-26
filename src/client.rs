use crate::handler;
use crate::message as msg;
use crossbeam_channel::{select,unbounded};

use std::sync::Mutex;

pub struct Client {
    handler: Box<dyn handler::Handler>,
    inc_server_chan: crossbeam_channel::Receiver<String>,
    inc_bot_chan: crossbeam_channel::Receiver<String>,
    started: Mutex<bool>,
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
            started: Mutex::new(false),
        }
    }

    fn start(&self) -> Result<(),crossbeam_channel::RecvError>{
        // Make sure the start function is only executed once.
        {
            let mut started = self.started.lock().unwrap();
            if *started {
                return Ok(());
            }
            *started = true;
        }

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
        let output_destination = handler::Outputs::Server;
        let incomming_message = r#"{"type": "Connected"}"#;
        let expected_reply = r#"{"type":"Register","clientType":"bot","game":"test_game","name":"test_bot"}"#;

        create_client_and_handle_message(incomming_message, output_destination, expected_reply)
    }

    #[test]
    fn receive_state_message_and_pass_on_to_bot() {
        let output_destination = handler::Outputs::Bot;
        let incomming_message = r#"{"type": "State", "other": "fields"}"#;
        let expected_reply = r#"{"type": "State", "other": "fields"}"#;

        create_client_and_handle_message(incomming_message, output_destination, expected_reply)
    }

    #[test]
    fn receive_error_message_and_pass_on_to_bot() {
        let output_destination = handler::Outputs::Bot;
        let incomming_message = r#"{"type": "Error", "message": "string"}"#;
        let expected_reply = r#"{"type": "Error", "message": "string"}"#;

        create_client_and_handle_message(incomming_message, output_destination, expected_reply)
    }

    #[test]
    fn receive_action_message_and_pass_on_to_server() {
        let output_destination = handler::Outputs::Server;
        let incomming_message = r#"{"type": "Action", "message": "string"}"#;
        let expected_reply = r#"{"type": "Action", "message": "string"}"#;

        create_client_and_handle_message(incomming_message, output_destination, expected_reply)
    }

    fn create_client_and_handle_message(msg_to_send: &str, output_type: handler::Outputs, expected_reply: &str) {
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