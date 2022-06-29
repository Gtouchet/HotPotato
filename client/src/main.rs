mod responses;
mod messages;
mod service;

use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json;
use crate::messages::*;
use crate::service::*;

fn main()
{
    let mut service = Service {
        stream: match TcpStream::connect("localhost:7878") {
            Ok(stream) => stream,
            Err(e) => {
                println!("Could not connect to the server: {}", e);
                return;
            }
        }
    };

    let mut message = Message::Hello;
    let mut serialized_message = serde_json::to_string(&message).unwrap();
    let result_from_hello = service.send_message(&serialized_message);
    println!("1. {:?}", result_from_hello);

    let subscribe : Subscribe = Subscribe { name: "free_potato".to_string() };
    serialized_message = serde_json::to_string(&Message::Subscribe(subscribe)).unwrap();
    let result_from_subscribe = service.send_message(&serialized_message);

    message = MessageParser::from_string(&result_from_subscribe);
    let subscription_result : SubscribeResult = match message {
        Message::SubscribeResult(subscribe_result) => subscribe_result,
        _ => panic!("expected SubscribeResult")
    };
    match subscription_result {
        SubscribeResult::Ok => println!("2. Ok"),
        SubscribeResult::Err(err) => println!("2. Err: {}", err)
    }
}
