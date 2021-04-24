use std::fmt;
use std::collections::HashMap;

use crate::message as msg;

#[derive(Hash,PartialEq,Eq,Debug,Clone)]
pub enum Outputs {
    Server,
    Bot,
}

pub enum Response {
    SetID(i32),
    Empty
}

#[derive(Debug)]
pub struct HandleError{
    msg: String,
}

impl fmt::Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl HandleError {
    fn unknown(message: msg::Message) -> HandleError {
        HandleError{
            msg: format!("unknown message type: {:?}", message).to_string()
        }
    }

    fn undefined_output(output: &Outputs) -> HandleError {
        HandleError{
            msg: format!("no output defined for {:?}", output).to_string()
        }
    }

    fn serialize(msg_type: &str) -> HandleError {
        HandleError{
            msg: format!("unmarshal {} message", msg_type).to_string()
        }
    }

    fn send(msg: String) -> HandleError {
        HandleError{
            msg: format!("send message: {}", msg).to_string()
        }
    }
}

pub trait Handler {
    fn handle(&self, json: String, msg_type: msg::Message) -> Result<Response,HandleError>;
    fn add_output_channel(&mut self,
        output_type: Outputs,
        channel: crossbeam_channel::Sender<String>);
}

pub struct MessageHandler {
    client_config: ClientConfig,
    outputs: HashMap<Outputs, crossbeam_channel::Sender<String>>,
}


impl Handler for MessageHandler {
    fn handle(&self, json: String, msg_type: msg::Message) -> Result<Response,HandleError> {
        match msg_type {
            // don't pass to client
            msg::Message::Connected(_) => self.handle_connected(),
            msg::Message::RegisterSuccess(rs) => self.handle_register_success(rs),
            // pass to client
            msg::Message::Error(_) => self.handle_error(json),
            msg::Message::State(_) => self.handle_state(json),
            // pass to server
            msg::Message::Action(_) => self.handle_action(json),
            _ => Err(HandleError::unknown(msg_type)),
        }
    }

    fn add_output_channel(&mut self, output_type: Outputs, channel: crossbeam_channel::Sender<String>){
        self.outputs.insert(output_type, channel);
    }
}

impl MessageHandler {
    pub fn new(client_config: ClientConfig) -> Self {
        MessageHandler{
            client_config: client_config,
            outputs: HashMap::new(),
        }
    }

    fn handle_connected(&self) -> Result<Response, HandleError> {
        let register_msg = msg::Message::Register(msg::Register {
            clientType: self.client_config.clientType.clone(),
            game: self.client_config.game.clone(),
            name: self.client_config.name.clone(),
        });

        let msg_string = msg::serialize_message(register_msg)
            .or_else(|_| Err(HandleError::serialize("register")))?;

        self.send(msg_string, &Outputs::Server)
    }

    fn handle_register_success(&self, m: msg::RegisterSuccess) -> Result<Response, HandleError> {
        Ok(Response::SetID(m.id))
    }

    fn handle_error(&self, m: String) -> Result<Response, HandleError> {
        self.send(m, &Outputs::Bot)
    }

    fn handle_state(&self, m: String) -> Result<Response, HandleError> {
        self.send(m, &Outputs::Bot)
    }

    fn handle_action(&self, m: String) -> Result<Response, HandleError> {
        self.send(m, &Outputs::Server)
    }

    fn send(&self, m: String, output: &Outputs) -> Result<Response, HandleError> {
        let chan = self.outputs.get(output)
            .ok_or(HandleError::undefined_output(output))?;

        chan.send(m)
            .map_err(|e| HandleError::send(format!("{}",e)))
            .map(|_| Response::Empty)
    }
}

pub struct ClientConfig{
    pub clientType: String,
    pub game: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    fn default_client_config() -> ClientConfig {
        ClientConfig{
            clientType: "bot".to_string(),
            game: "test_game".to_string(),
            name: "test_bot".to_string(),
        }
    }

    struct HandleResult{
        channel_response: String,
        response: Response,
    }

    impl HandleResult {
        fn assert_channel_response_equals(&self, s: String) {
            assert_eq!(self.channel_response, s)
        }

        fn assert_response_is_Empty(&self) {
            assert!(matches!(self.response, Response::Empty))
        }
    }

    fn handle_message_and_get_results(
            msg_json: String,
            target_output_channel: Outputs) -> HandleResult {

        let mut handler = MessageHandler::new(default_client_config());

        let (sender,receiver) = crossbeam_channel::bounded(1);
        handler.add_output_channel(target_output_channel, sender);

        let msg_obj = msg::deserialize_message(&msg_json).unwrap();
        let response = handler.handle(msg_json, msg_obj).unwrap();

        let channel_response = receiver.recv().unwrap();

        HandleResult{
            channel_response: channel_response,
            response: response
        }
    }

    fn handle_message_as_proxy_and_expect_empty_response(input_msg_string: String, target_output_channel: Outputs) {
        let result = handle_message_and_get_results(input_msg_string.clone(), target_output_channel);
        result.assert_channel_response_equals(input_msg_string);
        result.assert_response_is_Empty();
    }


    #[cfg(test)]
    mod nonproxy {
        use super::*;

        #[test]
        fn handle_msg_register_success_receive_SetID_response() {
            let msg_reg_suc = msg::Message::RegisterSuccess(
                msg::RegisterSuccess{
                    id: 1
                }
            );

            let mut handler = MessageHandler::new(default_client_config());
            let response = handler.handle("".to_string(), msg_reg_suc).unwrap();

            let ok = match response {
                Response::SetID(1) => true,
                _ => false,
            };
            assert_eq!(ok, true);
        }

        #[test]
        fn handle_msg_connected_get_empty_response_and_send_register() {
            let message_json = r#"{"type": "Connected"}"#.to_string();
            let response_message = msg::Message::Register(msg::Register{
                clientType: default_client_config().clientType,
                game: default_client_config().game,
                name: default_client_config().name,
            });
            let expected_channel_response = msg::serialize_message(response_message).unwrap();

            let target_output_channel = Outputs::Server;

            let result = handle_message_and_get_results(message_json, target_output_channel);
            result.assert_channel_response_equals(expected_channel_response);
            result.assert_response_is_Empty();
        }
    }

    #[cfg(test)]
    mod proxy {
        use super::*;

        #[test]
        fn handle_error_as_proxy() {
            let message_json = r#"{"type": "Error"}"#;
            let target_output_channel = Outputs::Bot;

            handle_message_as_proxy_and_expect_empty_response(
                message_json.to_string(),
                target_output_channel);
        }

        #[test]
        fn handle_state_as_proxy() {
            let message_json = r#"{"type": "State"}"#;
            let target_output_channel = Outputs::Bot;

            handle_message_as_proxy_and_expect_empty_response(
                message_json.to_string(),
                target_output_channel);
        }

        #[test]
        fn handle_action_as_proxy() {
            let message_json = r#"{"type": "Action"}"#;
            let target_output_channel = Outputs::Server;

            handle_message_as_proxy_and_expect_empty_response(
                message_json.to_string(),
                target_output_channel);
        }

    }

    #[cfg(test)]
    mod errors {
        use super::*;

        #[test]
        fn trigger_unknown_error_by_handling_register_message() {
            let register_msg = msg::Register {
                clientType: "x".to_string(),
                game: "y".to_string(),
                name: "z".to_string(),
            };
            let msg_json = msg::serialize_message(msg::Message::Register(register_msg.clone())).unwrap();

            let handler = MessageHandler::new(default_client_config());

            let response = handler.handle(msg_json, msg::Message::Register(register_msg.clone()));

            let returned_err = response.err().unwrap();
            let expected_err = HandleError::unknown(msg::Message::Register(register_msg));

            assert_eq!(returned_err.msg, expected_err.msg)
        }

        #[test]
        fn trigger_undefined_output_error_by_not_adding_output_channel() {
            let msg_json = msg::serialize_message(msg::Message::Connected(msg::Connected{})).unwrap();

            let handler = MessageHandler::new(default_client_config());

            let response = handler.handle(msg_json, msg::Message::Connected(msg::Connected{}));

            let returned_err = response.err().unwrap();
            let expected_err = HandleError::undefined_output(&Outputs::Server);

            assert_eq!(returned_err.msg, expected_err.msg)
        }

        #[test]
        fn trigger_send_error_by_closing_receiver_channel() {
            let msg_json = msg::serialize_message(msg::Message::Connected(msg::Connected{})).unwrap();

            let mut handler = MessageHandler::new(default_client_config());

            // Drop receiver to close sender channel
            let (sender,_) = crossbeam_channel::bounded(0);
            handler.add_output_channel(Outputs::Server, sender);

            let response = handler.handle(msg_json, msg::Message::Connected(msg::Connected{}));

            let returned_err = response.err().unwrap();
            let expected_err = HandleError::send("sending on a disconnected channel".to_string());

            assert_eq!(returned_err.msg, expected_err.msg)
        }
    }
}