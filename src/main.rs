use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::vec;

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

pub fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let parsed_request: Vec<&str> = request.split_whitespace().collect(); 
    
    if parsed_request[1] == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    } else {
        stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
    }
    println!("Request: {}", request);
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
}

pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, status_text: String, body: String) -> Response {
        Response {
            status_code, 
            status_text,
            headers: vec![
                ("Content-Type".to_string(), "text/plain".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
            body,
        }
    }

    pub fn add_headers(mut self, name: String, value: String) {
        self.headers.push((name, value));
    }


    pub fn to_string(&self) -> String {
        let mut header_string = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);

        for (name, value) in &self.headers {
            header_string.push_str(&format!("{}: {}\r\n", name, value));
        }

        header_string.push_str("\r\n");
        header_string.push_str(&self.body);

        header_string
    }
}