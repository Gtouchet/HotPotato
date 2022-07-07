use std::net::{SocketAddr, TcpListener};

fn main() {
    println!("server starting...");

    // Create a new TCP listener on the given address.
    let address = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(address);

    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Failed to bind to address : {err:?}"),
    };

    // Showing any message sent from client to server.
    for message in listener.incoming() {
        println!("message={message:?}");
    }
}
