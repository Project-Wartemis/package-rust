use serde_json::Value;
use serde::{Deserialize, Serialize};


pub type RawMessage = String;

#[serde(tag = "type")]
#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum XMessage {
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

	fn reserialize(msg: XMessage) {
		let json = serde_json::to_string(&msg).unwrap();
		let target: XMessage = serde_json::from_str(&json).unwrap();

		assert_eq!(target, msg);
	}

    #[test]
    fn test_action_message() {
		let message = XMessage::Action{
			game: 4882,
			key: "key".to_string(),
			action: json!({"This can be": "anything"}),
		};
		reserialize(message);
	}

	#[test]
    fn test_error_message() {
		let message = XMessage::Error{};
		reserialize(message);
    }

	#[test]
	fn test_action_message_engine() {
	let message = XMessage::EngineAction{
			game: 4882,
			player: "key".to_string(),
			action: json!({"This can be": "anything"}),
		};
		reserialize(message);
	}

	#[test]
	fn test_register_message() {
	let message = XMessage::Register{
			clientType: "clientType".to_string(),
			game: "game".to_string(),
			name: "name".to_string(),
		};
		reserialize(message);
	}

	#[test]
	fn test_start_message() {
	let message = XMessage::Start{
			game: 42,
			players: vec![0,1],
			prefix: "prefix".to_string(),
			suffix: "suffix".to_string(),
		};
		reserialize(message);
	}

	#[test]
	fn test_state_message() {
	let message = XMessage::State{
			game: 42,
			key: "key".to_string(),
			turn: 0,
			r#move: true,
			state : json!({"This can be": "anything"})
		};
		reserialize(message);
	}

	#[test]
	fn test_state_message_engine() {
	let message = XMessage::EngineState{
			game: 42,
			turn: 0,
			state : json!({"This can be": "anything"}),
			players: vec!["p1".to_string(),"p2".to_string()],

		};
		reserialize(message);
	}

	#[test]
	fn test_stop_message() {
	let message = XMessage::Stop{
			game: 42,
		};
		reserialize(message);
	}
}