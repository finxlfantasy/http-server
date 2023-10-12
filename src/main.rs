// Importing necessary modules
use std::{collections::HashMap, env, fs::{self, File}, io::prelude::*, net::{TcpListener, TcpStream}, str, thread};

// Function to parse the request string into start line, header and body
fn parse_request(request_str: &str) -> (&str, &str, &str) {
    let parsed = request_str.splitn(2, "\r\n").collect::<Vec<_>>();
    let parsed_no_start_line = parsed[1].splitn(2, "\r\n\r\n").collect::<Vec<_>>();
    (parsed[0], parsed_no_start_line[0], parsed_no_start_line[1])
}

// Function to parse the start line into request method, request path and http version
fn parse_start_line(start_line_str: &str) -> (&str, &str, &str) {
    let parsed = start_line_str.split_whitespace().collect::<Vec<_>>();
    (parsed[0], parsed[1], parsed[2])
}

// Function to parse the header into a HashMap
fn parse_header(header_str: &str) -> HashMap<&str, &str> {
    header_str.split("\r\n").filter_map(|header_line| {
        let pair = header_line.split(": ").collect::<Vec<_>>();
        if pair.len() == 2 { Some((pair[0], pair[1].trim())) } else { None }
    }).collect()
}

// Function to handle the connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let request_size = stream.read(&mut buffer).unwrap();

    // If the request string is valid, parse it and handle the request
    if let Ok(request_str) = str::from_utf8(&buffer[..request_size]) {
        println!("request string:\n\n{:?}", request_str);
        let (start_line_str, header_str, body_str) = parse_request(request_str);
        let (request_method, request_path, _http_version) = parse_start_line(start_line_str);
        let headers = parse_header(header_str);

        // Generate the response based on the request path
        let response = if request_path == r"/" {
            "HTTP/1.1 200 OK\r\n\r\n".to_string()
        } else if request_path.starts_with("/echo/") {
            let random_string = request_path.split("/echo/").nth(1).unwrap();
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", random_string.len(), random_string)
        } else if request_path == "/user-agent" {
            println!("{:?}", headers);
            let user_agent = headers["User-Agent"];
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent)
        } else if request_path.starts_with("/files/") {
            let file_name = request_path.split("/files/").nth(1).unwrap();
            let file_path = format!("{}/{}", env::args().last().unwrap(), file_name);
            // If the request method is GET, read the file and return its content
            // If the request method is POST, create the file and write the body into it
            if request_method == "GET" {
                match fs::read_to_string(&file_path) {
                    Ok(file_content) => format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", file_content.len(), file_content),
                    Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                }
            } else if request_method == "POST" {
                let mut file = File::create(&file_path).unwrap();
                file.write_all(body_str.as_bytes()).unwrap();
                "HTTP/1.1 201 OK\r\n\r\n".to_string()
            } else {
                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        } else {
            "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
        };
        // Write the response into the stream
        stream.write(response.as_bytes()).unwrap();
    }
}

// Main function
fn main() -> anyhow::Result<()> {
    // Bind the listener to the address and port
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // For each incoming stream, spawn a new thread to handle the connection
    for stream in listener.incoming() {
        if let Ok(_stream) = stream {
            thread::spawn(|| handle_connection(_stream));
        }
    }
    Ok(())
}
