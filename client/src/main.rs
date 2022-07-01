mod messages;
mod client;
mod random;
mod service;

use std::net::TcpStream;
use crate::client::Client;
use crate::messages::{Message, MessageParser};
use crate::random::Random;
use crate::service::Service;

fn main()
{
    let mut client = Client {
        service: Service {
            stream: connect_to_server("localhost:7878"),
        },
        random: Random {
            random: rand::thread_rng(),
        },
    };

    client.say_hello();

    let _ = client.subscribe(); // TODO: match

    loop
    {
        let mut players_list = Vec::new();

        let message_from_server = client.listen_to_server_message().unwrap();

        match MessageParser::from_string(&message_from_server) {
            Message::EndOfGame(end_of_game) => {
                println!("END OF GAME -- {:?}", end_of_game);
                break;
            }
            Message::RoundSummary(round_summary) => {
                client.display_round_summary(round_summary);
            }
            Message::Challenge(challenge) => {
                client.handle_challenge(challenge, players_list);
            }
            Message::PublicLeaderBoard(players) => {
                players.iter().for_each(|p| players_list.push(&p.name));
                client.display_leaderboard(players);
            }
            _ => {
                println!("Error: unexpected message wtf I quit bye");
                return;
            }
        }
    }
}

fn connect_to_server(address : &str) -> TcpStream
{
    let challenge_answer : ChallengeAnswer= match challenge {
        Challenge::RecoverSecret(_input) => {
            let recover_secret_result = ChallengeAnswer::RecoverSecret(RecoverSecretOutput {
                secret_sentence: "".to_string()
            }) ;
            recover_secret_result           
    match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(err) => {
            panic!("Could not connect to the server: {}", err);
        }
        Challenge::MD5HashCash(_input) => {
            let md5_result = ChallengeAnswer::MD5HashCash(MD5HashCashOutput {
                seed: 1,
                hashcode: "".to_string(),
            }) ;
            md5_result
        }
    };
    let challenge_result = ChallengeResult {
        next_target: players_list[random.get_number(0, players_list.len() - 1)].to_string(),
        result: challenge_answer
    };

    let serialized_message = serde_json::to_string(&Message::ChallengeResult(challenge_result)).unwrap();
    service.send_message(&serialized_message).unwrap();
    }
}
