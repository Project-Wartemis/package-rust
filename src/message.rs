use serde_json::Value;
use serde::{Deserialize, Serialize};

type Action = Value;
type State = Value;

#[derive(Deserialize, Serialize)]
struct ActionMessage {
	#[serde(rename = "type")]
	msgType: String,
	game: i32,
	key: String,
	action: Action,
}

#[derive(Deserialize, Serialize)]
struct ActionMessageEngine {
	#[serde(rename = "type")]
	msgType: String,
	game: i32,
	player: String,
	action: Action,
}

#[derive(Deserialize, Serialize)]
struct ErrorMessage {
	#[serde(rename = "type")]
	msgType: String,
}

#[derive(Deserialize, Serialize)]
struct RegisterMessage {
	#[serde(rename = "type")]
	msgType: String,
	#[serde(rename = "clientType")]
	client_type: String,
	game: String,
	name: String,
}

#[derive(Deserialize, Serialize)]
struct StartMessage {
	#[serde(rename = "type")]
	msgType: String,
	game: i32,
	players: Vec<i32>,
	prefix: String,
	suffix: String,
}

#[derive(Deserialize, Serialize)]
struct StateMessage {
	#[serde(rename = "type")]
	msgType: String,
	game: i32,
	key: String,
	turn: i32,
	r#move: bool,
	state: State,
}

#[derive(Deserialize, Serialize)]
struct StateMessageEngine {
	#[serde(rename = "type")]
	msgType: String,
	game: i32,
	turn: i32,
	players: Vec<String>,
	state: State,
}

#[derive(Deserialize, Serialize)]
struct StopMessage {
	#[serde(rename = "type")]
	msgType: String,
	game: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

	fn reserialize(message: &impl serde::Serialize, json: &str) -> bool {
		if let Ok(serialized) = serde_json::to_string(message){
				if serialized == json {
					return true;
				}
				println!("Result:   {}", serialized);
				println!("Expected: {}", json);
		}
		return false;
	}

    #[test]
    fn test_action_message() {
		let json = r#"{"type":"MSG","game":42,"key":"KEY","action":{"TEST":"TEST"}}"#;
        let message: ActionMessage = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.key,"KEY");
		assert_eq!(message.game,42);
    }

	#[test]
    fn test_action_message_engine() {
		let json = r#"{"type":"MSG","game":42,"player":"PLAYER","action":{"TEST":"TEST"}}"#;
        let message: ActionMessageEngine = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.player,"PLAYER");
		assert_eq!(message.game,42);
    }

	#[test]
    fn test_error_message() {
		let json = r#"{"type":"ERROR"}"#;
        let message: ErrorMessage = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"ERROR");
    }

	#[test]
    fn test_register_message() {
		let json = r#"{"type":"MSG","clientType":"CLIENTTYPE","game":"GAMENAME","name":"NAME"}"#;
        let message: RegisterMessage = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.game,"GAMENAME");
		assert_eq!(message.client_type,"CLIENTTYPE");
		assert_eq!(message.name,"NAME");
    }

	#[test]
    fn test_start_message() {
		let json = r#"{"type":"MSG","game":42,"players":[0,1],"prefix":"PREFIX","suffix":"SUFFIX"}"#;
        let message: StartMessage = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.game,42);
		assert_eq!(message.players,[0,1]);
		assert_eq!(message.prefix,"PREFIX");
		assert_eq!(message.suffix,"SUFFIX");
    }

	#[test]
    fn test_state_message() {
		let json = r#"{"type":"MSG","game":42,"key":"KEY","turn":0,"move":true,"state":{"STATE":{}}}"#;
        let message: StateMessage = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.game,42);
		assert_eq!(message.key,"KEY");
		assert_eq!(message.turn,0);
		assert_eq!(message.r#move,true);
    }

	#[test]
    fn test_state_message_engine() {
        let json = r#"{"type":"MSG","game":42,"turn":0,"players":["p1","p2"],"state":{"STATE":{}}}"#;
		let message: StateMessageEngine = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.game,42);
		assert_eq!(message.turn,0);
		assert_eq!(message.players,["p1","p2"]);
    }


	#[test]
    fn test_stop_message() {
		let json = r#"{"type":"MSG","game":42}"#;
        let message: StopMessage = serde_json::from_str(json).unwrap();
		assert_eq!(reserialize(&message, json), true);

		assert_eq!(message.msgType,"MSG");
		assert_eq!(message.game,42);
    }
}