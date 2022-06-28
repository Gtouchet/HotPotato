use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("connexion ok");

            let mut service = Service {
                stream,
            };

            let res = service.send_message("\"Hello\"");
            println!("response: {}", res);

            let res = service.send_message("{\"Subscribe\":{\"name\":\"free_potato\"}}");
            println!("response: {}", res);
        }
        Err(e) => {
            println!("connexion niquée");
        }
    }
}

struct Service {
    stream: TcpStream,
}

impl Service {
    fn send_message(&mut self, message: &str) -> String {
        println!("sending: {}", message);
        let message_size = message.len() as u32;
        self.stream.write_all(&message_size.to_be_bytes());
        self.stream.write_all(&message.as_bytes());
        let mut response = String::new();
        self.stream.read_to_string(&mut response);
        response
    }
}
