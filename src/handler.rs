use std::fmt;
use std::collections::HashMap;
// use crossbeam_channel;

use crate::message as msg;

#[derive(Hash,PartialEq,Eq,Debug)]
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

    fn send() -> HandleError {
        HandleError{
            msg: "failed to send message".to_string()
        }
    }
}

pub trait Handler {
    fn handle(&self, m: msg::Message) -> Result<Response, HandleError>;
    fn add_output_channel(&mut self,
        output_type: Outputs,
        channel: crossbeam_channel::Sender<String>);
}

pub struct ServerMessageHandler {
    client_config: ClientConfig,
    outputs: HashMap<Outputs, crossbeam_channel::Sender<String>>,
}


impl Handler for ServerMessageHandler {
    fn handle(&self, m: msg::Message) -> Result<Response,HandleError> {
        match m {
            // don't pass to client
            msg::Message::Connected(c) => self.handle_connected(c),
            msg::Message::RegisterSuccess(rs) => self.handle_register_success(rs),
            // pass to client
            msg::Message::Error(e) => self.handle_error(e),
            msg::Message::State(s) => self.handle_state(s),
            // pass to server
            msg::Message::Action(s) => self.handle_action(s),
            _ => Err(HandleError::unknown(m)),
        }
    }

    fn add_output_channel(&mut self, output_type: Outputs, channel: crossbeam_channel::Sender<String>){
        self.outputs.insert(output_type, channel);
    }
}

impl ServerMessageHandler {
    pub fn new(client_config: ClientConfig) -> Self {
        ServerMessageHandler{
            client_config: client_config,
            outputs: HashMap::new(),
        }
    }

    fn handle_connected(&self, _: msg::Connected) -> Result<Response, HandleError> {
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

    fn handle_error(&self, m: msg::Error) -> Result<Response, HandleError> {
        let msg_string = msg::serialize_message(msg::Message::Error(m))
            .or_else(|_| Err(HandleError::serialize("error")))?;
        self.send(msg_string, &Outputs::Bot)
    }

    fn handle_state(&self, m: msg::State) -> Result<Response, HandleError> {
        let msg_string = msg::serialize_message(msg::Message::State(m))
            .or_else(|_| Err(HandleError::serialize("error")))?;
        self.send(msg_string, &Outputs::Bot)
    }

    fn handle_action(&self, m: msg::Action) -> Result<Response, HandleError> {
        let msg_string = msg::serialize_message(msg::Message::Action(m))
            .or_else(|_| Err(HandleError::serialize("error")))?;
        self.send(msg_string, &Outputs::Server)
    }

    fn send(&self, m: String, output: &Outputs) -> Result<Response, HandleError> {
        let chan = self.outputs.get(output)
            .ok_or(HandleError::undefined_output(output))?;

        chan.send(m)
            .map_err(|_| HandleError::send())
            .map(|_| Response::Empty)
    }
}

pub struct ClientConfig{
    pub clientType: String,
    pub game: String,
    pub name: String,
}

#[cfg(test)]
mod server_message_handler {
    use super::*;
    use std::thread;

    fn default_client_config() -> ClientConfig {
        ClientConfig{
            clientType: "bot".to_string(),
            game: "test_game".to_string(),
            name: "test_bot".to_string(),
        }
    }

    fn handle_message_as_proxy(input_message: msg::Message, response_message: msg::Message, target_output_channel: Outputs) {
        let mut handler = ServerMessageHandler::new(default_client_config());

        let (sender,receiver) = crossbeam_channel::bounded(0);
        handler.add_output_channel(target_output_channel, sender);

        let listen_thread = thread::spawn(move || receiver.recv());

        let work_result = handler.handle(input_message).unwrap();
        let return_value_ok = match  work_result{
            Response::Empty => true,
            _ => false,
        };
        assert_eq!(return_value_ok, true);

        let resp_to_bot = listen_thread.join().unwrap().unwrap();

        assert_eq!(
            msg::deserialize_message(&resp_to_bot[..]).unwrap(),
            response_message)
    }

    #[test]
    fn handle_message_connected_as_proxy() {
        let input_message = msg::Message::Connected(msg::Connected{});
        let response_message = msg::Message::Register(msg::Register{
            clientType: default_client_config().clientType,
            game: default_client_config().game,
            name: default_client_config().name,
        });
        let target_output_channel = Outputs::Server;

        handle_message_as_proxy(input_message, response_message, target_output_channel);
    }

    #[test]
    fn handle_message_register_success() {
        let msg_reg_suc = msg::Message::RegisterSuccess(
            msg::RegisterSuccess{
                id: 1
            }
        );

        let mut handler = ServerMessageHandler::new(default_client_config());
        let response = handler.handle(msg_reg_suc).unwrap();

        let ok = match response {
            Response::SetID(1) => true,
            _ => false,
        };
        assert_eq!(ok, true);
    }

    #[test]
    fn handle_message_error_as_proxy() {
        let generate_error_msg = || {
            msg::Message::Error(
                msg::Error{
                    message: "example error".to_string(),
                }
            )
        };

        let input_message = generate_error_msg();
        let response_message = generate_error_msg();
        let target_output_channel = Outputs::Bot;

        handle_message_as_proxy(input_message, response_message, target_output_channel);
    }


    #[test]
    fn handle_message_state_as_proxy() {
        let generate_state_msg = || {
            msg::Message::State(
                msg::State{
                    game: 1,
                    key: "key".to_string(),
                    turn: 1,
                    r#move: true,
                    state: msg::JsonState::Null,
                }
            )
        };

        let input_message = generate_state_msg();
        let response_message = generate_state_msg();
        let target_output_channel = Outputs::Bot;

        handle_message_as_proxy(input_message, response_message, target_output_channel);
    }

    #[test]
    fn handle_message_action_as_proxy() {
        let generate_action_msg = || {
            msg::Message::Action(
                msg::Action{
                    game: 1,
                    key: "key".to_string(),
                    action: msg::JsonState::Null,
                }
            )
        };

        let input_message = generate_action_msg();
        let response_message = generate_action_msg();
        let target_output_channel = Outputs::Server;

        handle_message_as_proxy(input_message, response_message, target_output_channel);
    }
}