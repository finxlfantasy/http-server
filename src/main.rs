use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs;


fn handle_request(path: &str, client: &mut std::net::TcpStream) {
    let response = if path == "/" {
    // Create n HTTP response with staatus 200 OK and an empty resons body
        "HTTP/1.1 200 OK\r\n\r\nWelcome to the root again!"
    }   else {
        "HTTP/1.1 404 Not Found\r\nPage not found" 
    };
    // Writing the response to the client
    client.write_all(response.as_bytes()).unwrap();
    // Ensure te response is sent immediately by flushing the buffer
    client.flush().unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut client) => {
                println!("accepted new connection");

            // Read the HTTP request
            let mut request = Vec::new();
            client.read_to_end(&mut request).unwrap();
            let request_str = String::from_utf8_lossy(&request); 

            //Parsing the HTTP response to extract the Path
            let path = match request_str.lines().next() {
                Some(line) => {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        parts[1]
                    }   else {
                        "/"
                    }
                }
                None => "/",
            };

                handle_request(path, &mut client);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
 