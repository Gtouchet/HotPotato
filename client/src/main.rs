use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    let mut stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let test = send_message("\"Hello\"", stream);
            print!("{}", test);
        }
        Err(err) => panic!("Failed to connect to address : {err:?}")
    }
}

fn send_message(message: &str, mut stream: TcpStream) -> String {
    let message_size = message.len() as u32;
    stream.write_all(&message_size.to_be_bytes());
    stream.write_all(message.as_bytes());
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}
