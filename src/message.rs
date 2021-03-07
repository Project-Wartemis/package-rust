use serde_json::Value;
use serde::{Deserialize, Serialize};


#[serde(tag = "type")]
#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
	Connected		{},
	RegisterSuccess {id: i32},
	Action			{ game: i32, key: String, action: Action},
	Error			{ },
	Register		{ clientType: String, game: String, name: String},
	Start			{ game: i32, players: Vec<i32>, prefix: String, suffix: String},
	State 			{ game: i32, key: String, turn: i32, r#move: bool, state: State},
	Stop 			{ game: i32},
	EngineAction	{game: i32, player: String, action: Action},
	EngineState		{game: i32, turn: i32, players: Vec<String>, state: State},
}

type Action = Value;
type State = Value;

#[cfg(test)]
mod tests {
    use super::*;
	use serde_json::json;

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

	// Connected
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
		Message::Connected{}
	}

	fn get_string_message_connected() -> String {
		r#"{"type": "Connected"}"#.to_string()
	}

	// RegisterSuccess
    #[test]
    fn message_register_success_reserialize() {
		let msg_struct = get_object_message_register_success();
		reserialize(msg_struct);
	}

	#[test]
	fn message_register_success_deserialize() {
		let msg_json = get_string_message_action();
		let msg_struct = get_object_message_action();
		deserialize_and_validate(msg_struct, &msg_json);
	}

	fn get_object_message_register_success() -> Message {
		Message::RegisterSuccess{ id: 4884 }
	}

	fn get_string_message_register_success() -> String {
		r#"{
			"type": "RegisterSuccess",
			"id": 4884,
		}"#.to_string()
	}

	// Action
    #[test]
    fn message_action_reserialize() {
		let msg_struct = get_object_message_action();
		reserialize(msg_struct);
	}

	#[test]
	fn message_action_deserialize() {
		let msg_json = get_string_message_action();
		let msg_struct = get_object_message_action();
		deserialize_and_validate(msg_struct, &msg_json);
	}

	fn get_object_message_action() -> Message {
		Message::Action{
			game: 4884,
			key: "key".to_string(),
			action: json!({"This can be": "anything"}),
		}
	}

	fn get_string_message_action() -> String {
		r#"{
			"type": "Action",
			"game": 4884,
			"key": "key",
			"action": {"This can be": "anything"}
		}"#.to_string()
	}

	// Error
	#[test]
    fn message_error_reserialize() {
		let msg_struct = get_object_message_error();
		reserialize(msg_struct);
    }

	#[test]
    fn message_error_deserialize() {
		let msg_json = get_string_message_error();
		let msg_struct = get_object_message_error();
		deserialize_and_validate(msg_struct, &msg_json);
    }

	fn get_object_message_error() -> Message {
		Message::Error{}
	}

	fn get_string_message_error() -> String {
		r#"{
			"type": "Error"
		}"#.to_string()
	}

	// Register
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
		Message::Register{
			game: "game".to_string(),
			name: "name".to_string(),
			clientType: "clientType".to_string(),
		}
	}

	fn get_string_message_register() -> String {
		r#"{
			"type": "Register",
			"game": "game",
			"name": "name",
			"clientType": "clientType"
		}"#.to_string()
	}

	// Start
	#[test]
	fn message_start_reserialize() {
	let msg_struct = get_object_message_start();
		reserialize(msg_struct);
	}

	#[test]
	fn message_start_deserialize() {
		let msg_json = get_string_message_start();
		let msg_struct = get_object_message_start();
		deserialize_and_validate(msg_struct, &msg_json)
	}

	fn get_object_message_start() -> Message {
		Message::Start{
			game: 42,
			players: vec![0,1],
			prefix: "prefix".to_string(),
			suffix: "suffix".to_string(),
		}
	}

	fn get_string_message_start() -> String {
		r#"{
			"type": "Start",
			"game": 42,
			"players": [0,1],
			"prefix": "prefix",
			"suffix": "suffix"
		}"#.to_string()
	}

	// State
	#[test]
	fn message_state_reserialize() {
		let msg_struct = get_object_message_state();
		reserialize(msg_struct);
	}

	#[test]
	fn message_state_deserialize() {
		let msg_json = get_string_message_state();
		let msg_struct = get_object_message_state();
		deserialize_and_validate(msg_struct, &msg_json)
	}

	fn get_object_message_state() -> Message {
		Message::State{
			game: 42,
			key: "key".to_string(),
			turn: 0,
			r#move: true,
			state : json!({"This can be": "anything"})
		}
	}

	fn get_string_message_state() -> String {
		r#"{
			"type": "State",
			"game": 42,
			"key": "key",
			"turn": 0,
			"move": true,
			"state": {"This can be": "anything"}
		}"#.to_string()
	}

	// Stop
	#[test]
	fn message_stop_reserialize() {
		let msg_struct = get_object_message_stop();
		reserialize(msg_struct);
	}

	#[test]
	fn message_stop_deserialize() {
		let msg_json = get_string_message_stop();
		let msg_struct = get_object_message_stop();
		deserialize_and_validate(msg_struct, &msg_json)
	}

	fn get_object_message_stop() -> Message {
		Message::Stop{game: 42}
	}

	fn get_string_message_stop() -> String {
		r#"{
			"type": "Stop",
			"game": 42
		}"#.to_string()
	}

	// ActionEngine
	#[test]
	fn message_action_engine_reserialize() {
		let message = get_object_message_action_engine();
		reserialize(message);
	}

	#[test]
	fn message_action_engine_deserialize() {
		let msg_json = get_string_message_action_engine();
		let msg_struct = get_object_message_action_engine();
		deserialize_and_validate(msg_struct, &msg_json)
	}

	fn get_object_message_action_engine() -> Message {
		Message::EngineAction{
			game: 4882,
			player: "key".to_string(),
			action: json!({"This can be": "anything"}),
		}
	}

	fn get_string_message_action_engine() -> String {
		r#"{
			"type": "EngineAction",
			"game": 4882,
			"player": "key",
			"action": {"This can be": "anything"}
		}"#.to_string()
	}

	// StateEngine
	#[test]
	fn message_state_engine_reserialize() {
		let msg_struct = get_object_message_state_engine();
		reserialize(msg_struct);
	}

	#[test]
	fn message_state_engine_deserialize() {
		let msg_json = get_string_message_state_engine();
		let msg_struct = get_object_message_state_engine();
		deserialize_and_validate(msg_struct, &msg_json)
	}

	fn get_object_message_state_engine() -> Message {
		Message::EngineState{
			game: 42,
			turn: 0,
			state : json!({"This can be": "anything"}),
			players: vec!["p1".to_string(),"p2".to_string()],
		}
	}

	fn get_string_message_state_engine() -> String {
		r#"{
			"type": "EngineState",
			"game": 42,
			"turn": 0,
			"state": {"This can be": "anything"},
			"players": ["p1", "p2"]
		}"#.to_string()
	}
}