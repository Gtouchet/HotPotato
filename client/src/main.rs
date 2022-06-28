use std::io::{self, Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    let stream = TcpStream::connect("localhost:7878").unwrap();
    let mut service = Service { stream };

    let result1 = service.send_message("\"Hello\"");
    print!("1. {}\n", result1);
    let result2 = service.send_message("{\"Subscribe\":{\"name\":\"free_patato\"}}");
    print!("2. {}\n", result2);
}

struct Service {
    stream: TcpStream,
}

impl Service {
    fn send_message(&mut self, message: &str) -> String {
        let message_size = message.len() as u32;
        self.stream.write_all(&message_size.to_be_bytes());
        self.stream.write_all(message.as_bytes());
        let mut response = String::new();
        self.stream.read_to_string(&mut response);
        response
    }
}


