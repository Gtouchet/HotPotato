use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};

#[test]
fn bind_error() {
    match TcpListener::bind("1.1.1.1:9999") {
        Ok(..) => panic!(),
        Err(e) => assert_eq!(e.kind(), ErrorKind::AddrNotAvailable),
    }
}

#[test]
fn connect_error() {
    match TcpStream::connect("0.0.0.0:1") {
        Ok(..) => panic!(),
        Err(e) => assert!(
            e.kind() == ErrorKind::ConnectionRefused
                || e.kind() == ErrorKind::InvalidInput
                || e.kind() == ErrorKind::AddrInUse
                || e.kind() == ErrorKind::AddrNotAvailable,
            "bad error: {} {:?}",
            e,
            e.kind()
        ),
    }
}
