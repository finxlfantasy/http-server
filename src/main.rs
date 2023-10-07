
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "--directory" {
        println!("Usage: {} --directory <directory>", args[0]);
        return;
    }

    let root_directory = &args[2];

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root_directory = root_directory.to_string(); // Clone for the thread
                thread::spawn(move || {
                    handle_request(stream, &root_directory);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: TcpStream, root_directory: &str) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let parsed_request: Vec<&str> = request.split_whitespace().collect();

    if parsed_request.len() != 3 {
        stream.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()).unwrap();
        return;
    }

    let method = parsed_request[0];
    let path = parsed_request[1];

    if method != "GET" {
        stream.write("HTTP/1.1 405 Method Not Allowed\r\n\r\n".as_bytes()).unwrap();
        return;
    }

    if path.starts_with("/files/") {
        let filename = &path[7..]; // Remove "/files/" prefix to get the filename
        let file_path = format!("{}/{}", root_directory, filename);

        if let Ok(file) = fs::read(file_path) {
            let content_length = file.len();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                content_length
            );
            stream.write(response.as_bytes()).unwrap();
            stream.write(&file).unwrap();
        } else {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        }
    } else {
        stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
    }
}
