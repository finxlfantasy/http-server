use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::vec;
use std::thread;
use std::env; 
use std::fs::File;
fn main() {
   let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "--directory" {
        println!("Usage: {} --directory <directory>", args[0]);
        return;
    }
    let directory = &args[2];
 
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory_clone = directory.to_string();
                thread::spawn(move || { handle_request(stream, &directory_clone);
            });   
            // ^ Spawns a new thread for each connection/request
        }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

pub fn handle_request(mut stream: TcpStream, directory: &str) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let parsed_request: Vec<&str> = request.split_whitespace().collect(); 


/*     if parsed_request.len() < 2 {
        stream.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()).unwrap();
        return;
    }
 */
    let requested_path = parsed_request[1];
    
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
/*     } else if requested_path.starts_with("/files/") {
        let file_path = format!("{}/{}", directory, &requested_path[7..]);
            if let Ok(file_content) = fs::read_to_string(&file_path) {
                let response = Response::new(200, "Ok".to_string(), file_content);
                stream.write(response.to_string().as_bytes()).unwrap(); */
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


    // adding headers 
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

fn extract_user_agent(request: &str) -> String {
    if let Some(start) = request.find("User-Agent: ") {
        if let Some(end) = request[start..].find("\r\n") {
            return request[start + 12..start + end].to_string();
        }
    }
    String::from("User-Agent header not found")
}