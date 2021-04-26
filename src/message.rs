use std::fmt;
use serde_json::Value;
use serde_json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error,Debug)]
pub enum MessageError {
	#[error("deserialise message: {source}")]
    Deserialize{source: serde_json::Error},

	#[error("serialise message: {source}")]
    Serialize{ source: serde_json::Error},
}

#[serde(tag = "type")]
#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
	Connected		(Connected),
	RegisterSuccess (RegisterSuccess),
	Register 		(Register),
	Action			(MessageContent),
	Error 			(MessageContent),
	State 			(MessageContent),
}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct MessageContent {
	#[serde(skip_serializing,flatten)]
	pub content: Value,
}

pub fn deserialize_message(json: &str) -> Result<Message, MessageError> {
	serde_json::from_str(json)
		.map_err(|e| MessageError::Deserialize{source: e})
}

pub fn serialize_message(message: Message) -> Result<String, MessageError> {
	serde_json::to_string(&message)
		.map_err(|e| MessageError::Serialize{source: e})
}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Connected {}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct RegisterSuccess {
	pub id: i32
}

#[derive(Serialize, Deserialize,Debug,PartialEq,Clone)]
pub struct Register {
	pub clientType: String,
	pub game: String,
	pub name: String
}

#[cfg(test)]
mod tests {
    use super::*;

	// Helper functions
	fn reserialize(msg: Message) {
		let json = serde_json::to_string(&msg).unwrap();
		let target: Message = serde_json::from_str(&json).unwrap();

		assert_eq!(target, msg);
	}

	fn deserialize_and_validate(msg_struct: Message, msg_json: &str) {
		let result: Message = serde_json::from_str(msg_json).unwrap();

		assert_eq!(result, msg_struct);
	}

	#[cfg(test)]
	mod general {
		use super::*;

		#[test]
		fn deserialising_invalid_json_should_return_an_error() {
			let err = deserialize_message("invalid_json");
			match err {
				Err(MessageError::Deserialize{..}) => (),
				_ => panic!("Expected an error when deserializing a json but got {:?}", err),
			}
		}

		#[test]
		fn deserialising_unknown_message_should_return_an_error() {
			let err = deserialize_message(r#"{"type": "Foo"}"#);
			match err {
				Err(MessageError::Deserialize{..}) => {},
				_ => panic!("Expected an error when deserializing a json but got {:?}", err),
			}
		}
	}

	#[cfg(test)]
	mod connected {
		use super::*;

		#[test]
		fn message_connected_reserialize() {
			let msg_struct = get_object_message_connected();
			reserialize(msg_struct);
		}

		#[test]
		fn message_connected_deserialize() {
			let msg_json = get_string_message_connected();
			let msg_struct = get_object_message_connected();
			deserialize_and_validate(msg_struct, &msg_json);
		}

		fn get_object_message_connected() -> Message {
			Message::Connected(Connected{})
		}

		fn get_string_message_connected() -> &'static str {
			r#"{"type": "Connected"}"#
		}
	}

	#[cfg(test)]
	mod register_success {
		use super::*;

		#[test]
		fn message_register_success_reserialize() {
			let msg_struct = get_object_message_register_success();
			reserialize(msg_struct);
		}

		#[test]
		fn message_register_success_deserialize() {
			let msg_json = get_string_message_register_success();
			let msg_struct = get_object_message_register_success();
			deserialize_and_validate(msg_struct, &msg_json);
		}

		fn get_object_message_register_success() -> Message {
			Message::RegisterSuccess(RegisterSuccess{ id: 4884 })
		}

		fn get_string_message_register_success() -> &'static str {
			r#"{
				"type": "RegisterSuccess",
				"id": 4884
			}"#
		}
	}


	#[cfg(test)]
	mod register {
		use super::*;

		#[test]
		fn message_register_reserialize() {
			let msg_struct = get_object_message_register();
			reserialize(msg_struct);
		}

		#[test]
		fn message_register_deserialize() {
			let msg_json = get_string_message_register();
			let msg_struct = get_object_message_register();
			deserialize_and_validate(msg_struct, &msg_json);
		}

		fn get_object_message_register() -> Message {
			Message::Register(Register{
				game: "game".to_string(),
				name: "name".to_string(),
				clientType: "clientType".to_string(),
			})
		}

		fn get_string_message_register() -> &'static str {
			r#"{
				"type": "Register",
				"game": "game",
				"name": "name",
				"clientType": "clientType"
			}"#
		}
	}


	#[cfg(test)]
	mod action {
		use super::*;

		#[test]
		fn message_action_reserialize() {
			let msg_struct = get_object_message_action();
			reserialize(msg_struct);
		}

		#[test]
		fn message_error_deserialize() {
			match deserialize_message(get_string_message_action()).unwrap() {
				Message::Action(..) => (),
				_ => panic!("deserialise action"),
			}
		}

		fn get_object_message_action() -> Message {
			let mut map = serde_json::Map::new();
			Message::Error(
				MessageContent{
					content: Value::Object(map)
				}
			)
		}

		fn get_string_message_action() -> &'static str {
			r#"{
				"type": "Action",
				"game": 4884,
				"key": "key",
				"action": {"This can be": "anything"},
				"Can even add other fields": "doesn't matter"
			}"#
		}
	}

	#[cfg(test)]
	mod error {
		use super::*;

		#[test]
		fn message_error_reserialize() {
			let msg_struct = get_object_message_error();
			reserialize(msg_struct);
		}

		#[test]
		fn message_error_deserialize() {
			match deserialize_message(get_string_message_error()).unwrap() {
				Message::Error(..) => (),
				_ => panic!("deserialise error"),
			}
		}

		fn get_object_message_error() -> Message {
			let mut map = serde_json::Map::new();
			Message::Error(
				MessageContent{
					content: Value::Object(map)
				}
			)
		}

		fn get_string_message_error() -> &'static str {
			r#"{
				"type": "Error",
				"message": "You messed up",
				"other fields": "anything"
			}"#
		}
	}

	#[cfg(test)]
	mod state {
		use super::*;

		#[test]
		fn message_state_reserialize() {
			let msg_struct = get_object_message_state();
			reserialize(msg_struct);
		}

		#[test]
		fn message_state_deserialize() {
			match deserialize_message(get_string_message_state()).unwrap() {
				Message::State(..) => (),
				_ => panic!("deserialise state"),
			}
		}

		fn get_object_message_state() -> Message {
			let mut map = serde_json::Map::new();
			Message::State(
				MessageContent{
					content: Value::Object(map)
				}
			)
		}

		fn get_string_message_state() -> &'static str {
			r#"{
				"type": "State",
				"key": "value",
				"this": "is ignored"
			}"#
		}
	}
}