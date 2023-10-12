use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::env; 
use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "--directory" {
        println!("Usage: {} --directory <directory>", args[0]);
        return;
    }

    let directory = PathBuf::from(&args[2]);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                thread::spawn(move || { handle_request(stream, &directory) });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

pub fn handle_request(mut stream: TcpStream, directory: &PathBuf) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let parsed_request: Vec<&str> = request.split_whitespace().collect(); 

    if parsed_request[1] == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    } else if parsed_request[1].starts_with("/echo") {
        let data = parsed_request[1].replace("/echo/", "");
        let response = Response::new(200, "Ok".to_string(), data);
        stream.write(response.to_string().as_bytes()).unwrap();
    } else if parsed_request[1] == "/user-agent" {
        let user_agent = extract_user_agent(&request);
        let response = Response::new(200, "Ok".to_string(), user_agent);
        stream.write(response.to_string().as_bytes()).unwrap();
    } else if parsed_request[1].starts_with("/files/") {
        let filename = &parsed_request[1][7..];
        let filepath = directory.join(filename);

        if filepath.exists() {
            let mut file = fs::File::open(&filepath).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let mut response = Response::new(200, "Ok".to_string(), contents);
            response.add_headers("Content-Type".to_string(), "application/octet-stream".to_string());
            stream.write(response.to_string().as_bytes()).unwrap();
        } else {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        }
    }
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

    pub fn add_headers(&mut self, name: String, value: String) {
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

fn extract_user_agent(request: &str) -> String {
    if let Some(start) = request.find("User-Agent: ") {
        if let Some(end) = request[start..].find("\r\n") {
            return request[start + 12..start + end].to_string();
        }
    }
    String::from("User-Agent header not found")
}