use std::io::Write;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message= b"Hello";
            let response = stream.write_all(message);
            print!("resp: {:?}", response);
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}
