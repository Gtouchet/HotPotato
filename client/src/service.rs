use std::net::TcpStream;
use std::io::{self, Read, Write};
pub struct Service {
    pub stream: TcpStream,
}

impl Service {
    pub fn send_message(&mut self, message: &str) -> String {
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