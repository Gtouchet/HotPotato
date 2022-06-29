use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
}

pub struct MessageParser {
}

impl MessageParser {
    pub fn from_string(string_to_parse: &str) -> Message {
        let response: Result<Message, serde_json::Error> = serde_json::from_str(&string_to_parse);
        let message: Message = match response {
            Ok(m) => m,
            Err(err) => panic!("cannot parse to Message : {err:?}")
        };
        message
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(String),
}

// #[derive(Debug, Serialize, Deserialize)]
// struct PublicLeaderBoard {
//     leaderboard: Vec<PublicPlayer>
// }