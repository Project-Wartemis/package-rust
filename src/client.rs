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
        let (server_inc_snd,     server_inc_rec) = crossbeam_channel::bounded(1);
        let (bot_inc_snd_unused, bot_inc_rec) = crossbeam_channel::bounded(1);

        let (server_out_snd,     server_out_rec) = crossbeam_channel::bounded(1);

        // Create and start client
        thread::spawn(move || {
            let mut handler = handler::MessageHandler::new(default_client_config());
            handler.add_output_channel(handler::Outputs::Server, server_out_snd);

            let c = Client::new(Box::new(handler), server_inc_rec, bot_inc_rec);

            let r = c.start();
            if let Err(e) = r {
                assert_eq!(format!("{}", e), "");
            }
        });

        // client receives Connected message from Server
        let input_msg = r#"{"type": "Connected"}"#.to_string();
        server_inc_snd.send(input_msg);

        // get reply
        let response = server_out_rec.recv().unwrap();
        let expected_response = r#"{"type":"Register","clientType":"bot","game":"test_game","name":"test_bot"}"#.to_string();
        assert_eq!(response, expected_response)
    }

    #[test]
    fn receive_state_message_and_pass_on_to_bot() {
        let (server_inc_snd,    server_inc_rec) = crossbeam_channel::bounded(1);
        let (bot_inc_snd_unused,                 bot_inc_rec) = crossbeam_channel::bounded(1);

        let (bot_out_snd, bot_out_rec) = crossbeam_channel::bounded(1);

        // Create and start client
        thread::spawn(move || {
            let mut handler = handler::MessageHandler::new(default_client_config());
            handler.add_output_channel(handler::Outputs::Bot, bot_out_snd);

            let c = Client::new(Box::new(handler), server_inc_rec, bot_inc_rec);

            let r = c.start();
            if let Err(e) = r {
                assert_eq!(format!("{}", e), "");
            }
        });

        // client receives Connected message from Server
        let input_msg = r#"{"type": "State", "other": "fields"}"#;
        server_inc_snd.send(input_msg.to_string());

        // get reply
        let response = bot_out_rec.recv().unwrap();
        let expected_response = input_msg.to_string();
        assert_eq!(response, expected_response)
    }

    #[test]
    fn receive_error_message_and_pass_on_to_bot() {
        let (server_inc_snd,    server_inc_rec) = crossbeam_channel::bounded(1);
        let (bot_inc_snd_unused,                 bot_inc_rec) = crossbeam_channel::bounded(1);

        let (bot_out_snd, bot_out_rec) = crossbeam_channel::bounded(1);

        // Create and start client
        thread::spawn(move || {
            let mut handler = handler::MessageHandler::new(default_client_config());
            handler.add_output_channel(handler::Outputs::Bot, bot_out_snd);

            let c = Client::new(Box::new(handler), server_inc_rec, bot_inc_rec);

            let r = c.start();
            if let Err(e) = r {
                assert_eq!(format!("{}", e), "");
            }
        });

        // client receives Connected message from Server
        let input_msg = r#"{"type": "Error", "message": "string"}"#;
        server_inc_snd.send(input_msg.to_string());

        // get reply
        let response = bot_out_rec.recv().unwrap();
        let expected_response = input_msg.to_string();
        assert_eq!(response, expected_response)
    }

    #[test]
    fn receive_action_message_and_pass_on_to_server() {
        let (server_inc_snd_unused,    server_inc_rec) = crossbeam_channel::bounded(1);
        let (bot_inc_snd,                 bot_inc_rec) = crossbeam_channel::bounded(1);

        let (server_out_snd, server_out_rec) = crossbeam_channel::bounded(1);

        // Create and start client
        thread::spawn(move || {
            let mut handler = handler::MessageHandler::new(default_client_config());
            handler.add_output_channel(handler::Outputs::Server, server_out_snd);

            let c = Client::new(Box::new(handler), server_inc_rec, bot_inc_rec);

            let r = c.start();
            if let Err(e) = r {
                assert_eq!(format!("{}", e), "");
            }
        });

        // client receives Connected message from Server
        let input_msg = r#"{"type": "Action", "message": "string"}"#;
        bot_inc_snd.send(input_msg.to_string());

        // get reply
        let response = server_out_rec.recv().unwrap();
        let expected_response = input_msg.to_string();
        assert_eq!(response, expected_response)
    }
}