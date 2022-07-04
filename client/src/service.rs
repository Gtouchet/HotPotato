use std::net::TcpStream;
use std::io::{Read, Write};

pub struct Service
{
    pub(crate) stream: TcpStream,
}

impl Service
{
    pub(crate) fn send_message(&mut self, message: &str)
    {
        let message_size = message.len() as u32;
        match self.stream.write_all(&message_size.to_be_bytes()) {
            Ok(_) => {},
            Err(err) => panic!("Error: could not write client message size\n{}", err),
        };
        match self.stream.write_all(message.as_bytes()) {
            Ok(_) => {},
            Err(err) => panic!("Error: could not write client message\n{}", err),
        };
    }

    pub(crate) fn listen_to_response(&mut self) -> String
    {
        let mut buffer: &mut[u8] = &mut [0; 4];
        match self.stream.read_exact(&mut buffer) {
            Ok(_) => {},
            Err(err) => panic!("Error: could not read server response size\n{}", err),
        };

        let response_message_size = u32::from_be_bytes(buffer.try_into().unwrap());
        let mut response_buffer = vec![0; response_message_size as usize];
        return match self.stream.read_exact(&mut response_buffer) {
            Ok(_) => String::from_utf8(response_buffer).unwrap(),
            Err(err) => panic!("Error: could not read server response\n{}", err),
        };
    }

    pub(crate) fn send_message_and_listen_to_response(&mut self, message: &str) -> String
    {
        self.send_message(message);
        return self.listen_to_response();
    }
}
