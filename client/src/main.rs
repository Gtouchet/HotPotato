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

    let message = Message::Hello;
    let mut serialized_message = serde_json::to_string(&message).unwrap();
    let result1 = service.send_message(&serialized_message);
    println!("1. {:?}", result1);

    let subscribe : Subscribe = Subscribe { name: "free_potato".to_string() };
    serialized_message = serde_json::to_string(&Message::Subscribe(subscribe)).unwrap();
    let result2 = service.send_message(&serialized_message);
    println!("2. {:?}", result2);
}
