
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    // Handle client connection
}