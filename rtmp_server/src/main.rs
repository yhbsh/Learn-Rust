// main.rs

use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("Client connected: {}", stream.peer_addr()?);
    stream.write_all(b"Hello from server")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1935")?;
    println!("Server listening on port 1935");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {:?}", e);
            }
        }
    }
    Ok(())
}
