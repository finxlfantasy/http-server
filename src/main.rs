use std::net::TcpListener;
use std::io::{Read, Write};
use std::str;

fn handle_request(path: &str, client: &mut std::net::TcpStream) {
    let response = if path == "/" {
        // Respond with 200 OK for the root path
        "HTTP/1.1 200 OK\r\n\r\nWelcome to the root page!"
    } else {
        // Respond with 404 Not Found for any other path
        "HTTP/1.1 404 Not Found\r\n\r\nPage not found"
    };

    // Write the response to the client
    client.write_all(response.as_bytes()).unwrap();
    client.flush().unwrap();
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut client) => {
                println!("Accepted new connection");

                // Read the HTTP request
                let mut request = Vec::new();
                client.read_to_end(&mut request).unwrap();
                let request_str = String::from_utf8_lossy(&request);

                // Parse the HTTP request to extract the path
                let path = match request_str.lines().next() {
                    Some(line) => {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            parts[1]
                        } else {
                            "/"
                        }
                    }
                    None => "/",
                };

                // Handle the request and respond accordingly
                handle_request(path, &mut client);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}