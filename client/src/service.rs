use std::net::TcpStream;
use std::io::{Error, Read, Write};
use std::string::FromUtf8Error;

pub struct Service
{
    pub(crate) stream: TcpStream,
}

impl Service
{
    pub(crate) fn send_message(&mut self, message: &str) -> Result<String, Error>
    {
        let message_size = message.len() as u32;
        self.stream.write_all(&message_size.to_be_bytes())?;
        self.stream.write_all(message.as_bytes())?;

        let mut buffer: &mut[u8] = &mut [0; 4];
        self.stream.read_exact(&mut buffer)?;
        let response_message_size = u32::from_be_bytes(buffer.try_into().unwrap());

        let mut response_buffer = vec![0; response_message_size as usize];
        self.stream.read_exact(&mut response_buffer)?;

        let response_as_string = String::from_utf8(response_buffer.try_into().unwrap()).unwrap();

        Ok(response_as_string)
    }
}
