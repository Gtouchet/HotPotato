mod responses;
mod messages;
mod service;

use std::net::TcpStream;
use serde_json;
use crate::messages::*;
use crate::service::*;

fn main() {
    let stream = match TcpStream::connect("localhost:7878") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Could not connect to the server: {}", e);
            return;
        }
    };
    let mut service = Service { stream };

    let message = Message::Hello;
    let mut serialized_message = serde_json::to_string(&message).unwrap();
    let result1 = service.send_message(&serialized_message);
    println!("resp 1. {}", result1);
    let response = serde_json::from_str(&result1);
    let welcome = match response {
        Ok(w) => w,
        Err(err) => panic!("cannot parse to Welcome : {err:?}")
    };
    println!("welcome: {:?}", welcome);

    let subscribe : Subscribe = Subscribe { name: "free_potato".to_string() };
    serialized_message = serde_json::to_string(&Message::Subscribe(subscribe)).unwrap();
    let result2 = service.send_message(&serialized_message);
    println!("resp 2. {}", result2);
}