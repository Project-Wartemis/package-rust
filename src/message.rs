use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MessageError {}

#[serde(tag = "type")]
#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
	Connected		(Connected),
	RegisterSuccess (RegisterSuccess),
	Register 		(Register),
	Action			(Action),
	Error 			(Error),
	Start 			(Start),
	State 			(State),
	Stop 			(Stop),
	EngineAction 	(EngineAction),
	EngineState 	(EngineState),
}

pub type JsonAction = Value;
pub type JsonState = Value;

pub fn deserialize_message(json: &str) -> Result<Message, MessageError> {
	let r = serde_json::from_str(json);
	match r {
		Ok(r) => Ok(r),
		Err(_) => Err(MessageError{}),
	}
}

pub fn serialize_message(message: Message) -> Result<String, MessageError> {
	match serde_json::to_string(&message) {
            Ok(r) => Ok(r),
            Err(_) => Err(MessageError{}),
        }
}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct RegisterSuccess {
	pub id: i32
}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Connected {}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Register {
	pub clientType: String,
	pub game: String,
	pub name: String
}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Action	{pub game: i32, pub key: String, pub action: JsonState}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Error 	{ pub message: String }

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Start 	{ pub game: i32, pub players: Vec<i32>, pub prefix: String, pub suffix: String}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct State 	{ pub game: i32, pub key: String, pub turn: i32, pub r#move: bool, pub state: JsonState}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct Stop 	{ pub game: i32}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct EngineAction {pub game: i32, pub player: String, pub action: JsonAction}

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub struct EngineState {pub game: i32, pub turn: i32, pub players: Vec<String>, pub state: JsonState}
