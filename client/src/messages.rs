use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe)
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
pub struct SubscribeResult {

}

// #[derive(Debug, Serialize, Deserialize)]
// struct PublicLeaderBoard {
//     leaderboard: Vec<PublicPlayer>
// }