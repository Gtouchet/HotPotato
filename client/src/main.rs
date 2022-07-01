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
    say_hello(&mut service);
    subscribe(&mut service);
    play(&mut service);    
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

fn say_hello(service: &mut Service) {
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

fn subscribe(service: &mut Service) {
    let mut random = Random { random: rand::thread_rng() };
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

fn play(service : &mut Service) {
    loop {
        let message_from_server = service.listen_to_server_message().unwrap();
        println!("loop. {}", message_from_server);
        
        let parsed_message = MessageParser::from_string(&message_from_server);
        match parsed_message {
            Message::EndOfGame(end_of_game) => {
                println!("3. end_of_game {:?}", end_of_game);
                break;
            }
            Message::RoundSummary(round_summary) => {
                println!("3. round_summary {:?}", round_summary);
            }
            Message::Challenge(challenge) => {
                println!("3. challenge {:?}", challenge);
            }
            Message::PublicLeaderBoard(players) => {
                println!("3. players {:?}", players);
            }
            _ => {
                println!("3. unexpected message");
                return;
            }
        }
    }
}