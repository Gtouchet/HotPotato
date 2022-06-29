mod responses;
mod messages;

use std::io::{self, Read, Write};
use std::net::TcpStream;
use serde_json;
use serde::{Serialize, Deserialize};
use crate::messages::*;

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

struct Service {
    stream: TcpStream,
}

impl Service {
    fn send_message(&mut self, message: &str) -> String {
        println!("send_message: {}", message);

        let message_size = message.len() as u32;
        match self.stream.write_all(&message_size.to_be_bytes()) {
            Ok(r) => {
                println!("\twrite_all size {}: ok", message_size);
            },
            Err(e) => {
                println!("\twrite_all size error: {}", e);
            }
        };

        match self.stream.write_all(message.as_bytes()) {
            Ok(r) => {
                println!("\twrite_all message ok");
            },
            Err(e) => {
                println!("\twrite_all message error: {}", e);
            }
        };

        let mut response = String::new();
        match self.stream.read_to_string(&mut response) {
            Ok(r) => {
                println!("\tread_to_string ok");
                response
            }
            Err(e) => {
                println!("\tread_to_string error: {}", e);
                String::new()
            }
        }
    }
}
