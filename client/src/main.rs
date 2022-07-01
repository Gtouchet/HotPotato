mod messages;
mod service;
mod random;
mod recoversecret;

use std::net::TcpStream;
use serde_json;
use crate::messages::*;
use crate::random::Random;
use crate::service::*;

fn main()
{
    let mut service = Service { stream : connect_to_server("localhost:7878")};
    let mut random = Random { random: rand::thread_rng() };

    say_hello(&mut service);
    subscribe(&mut service, &mut random);
    play(&mut service, &mut random);
}

fn connect_to_server(address : &str) -> TcpStream
{
    match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(e) => {
            panic!("Could not connect to the server: {}", e);
        }
    }
}

fn say_hello(service: &mut Service)
{
    let message = Message::Hello;
    let serialized_message = match serde_json::to_string(&message) {
        Ok(m) => m,
        Err(e) => {
            println!("Could not serialize the message: {}", e);
            return;
        }
    };
    let result1 = service.send_message(&serialized_message);
    println!("1. {:?}", result1);
}

fn subscribe(service: &mut Service, random: &mut Random)
{
    let subscribe: Subscribe = Subscribe { name: random.generate_name() };
    let serialized_message = serde_json::to_string(&Message::Subscribe(subscribe)).unwrap();
    let result_from_subscribe = service.send_message(&serialized_message);

    let message = MessageParser::from_string(&result_from_subscribe.unwrap());
    let subscription_result : SubscribeResult = match message {
        Message::SubscribeResult(subscribe_result) => subscribe_result,
        _ => panic!("expected SubscribeResult")
    };
    match subscription_result {
        SubscribeResult::Ok => println!("2. Ok"),
        SubscribeResult::Err(err) => {
            println!("2. Err: {}", err);
            return
        }
    }
}

fn play(service: &mut Service, random: &mut Random)
{
    loop
    {
        let mut players_list = Vec::new();

        let message_from_server = service.listen_to_server_message().unwrap();

        match MessageParser::from_string(&message_from_server) {
            Message::EndOfGame(end_of_game) => {
                println!("END OF GAME -- {:?}", end_of_game);
                break;
            }
            Message::RoundSummary(round_summary) => {
                display_round_summary(round_summary);
            }
            Message::Challenge(challenge) => {
                handle_challenge(service, challenge, players_list, random);
            }
            Message::PublicLeaderBoard(players) => {
                players.iter().for_each(|p| players_list.push(&p.name));
                display_leaderboard(players);
            }
            _ => {
                println!("Error: unexpected message wtf I quit");
                return;
            }
        }
    }
}

fn handle_challenge(service: &mut Service, challenge: Challenge, players_list: Vec<&String>, random: &mut Random)
{
    let challenge_answer : ChallengeAnswer= match challenge {
        Challenge::RecoverSecret(_input) => {
            let recover_secret_result = ChallengeAnswer::RecoverSecret(RecoverSecretOutput {
                secret_sentence: "".to_string()
            }) ;
            recover_secret_result           
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

fn display_leaderboard(players: Vec<PublicPlayer>)
{
    println!("----- Leaderboard -----\n");
    players.iter()
        .for_each(|p| println!(
            "Player {}:\n\
            - Score: {}\n\
            - Steps: {}\n\
            - Active: {}\n\
            - Used time: {}\n",
            p.name, p.score, p.steps, p.is_active, p.total_used_time)
        )
}

fn display_round_summary(round_summary: RoundSummary)
{
    // TODO
    println!("----- Round summary -----\n");
    println!("{:?}", round_summary);
}