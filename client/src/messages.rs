use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeResult {

}

// #[derive(Debug, Serialize, Deserialize)]
// struct PublicLeaderBoard {
//     leaderboard: Vec<PublicPlayer>
// }