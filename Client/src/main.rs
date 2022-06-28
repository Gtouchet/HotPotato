use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("client starting...");

    let mut stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            println!("connected to server");
            let message = "Hello".as_bytes();
            let response = stream.write_all(&message);

        }
        Err(err) => panic!("Failed to connect to address : {err:?}")
    }
}
