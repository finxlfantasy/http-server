use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

use anyhow::Result;

pub fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]).to_string();
    let parsed_request: Vec<&str> = request.split_whitespace().collect(); 
    if parsed_request[1] == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    } else {
        stream.write("HTTP/1.1 404 Not Found\r\nPage not found".as_bytes()).unwrap();
    }
    println!("Request: {}", request);
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_request(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
