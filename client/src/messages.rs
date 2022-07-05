use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

pub struct MessageParser {
}

impl MessageParser {
    pub(crate) fn from_string(string_to_parse: &str) -> Message {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard {
    pub leader_board: Vec<PublicPlayer>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,    
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    RecoverSecret(RecoverSecretInput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub seed: u64,
    // hashcode found using seed + message
    pub hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    pub leader_board: Vec<PublicPlayer>
}