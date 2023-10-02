use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut client) => {
                println!("accepted new connection");
                
            // Create n HTTP response with staatus 200 OK and an empty resons body
            let response = "HTTP/1.1 200 OK\r\n\r\n";


            // Writing the response to the client
            client.write_all(response.as_bytes()).unwrap();

            // Ensure te response is sent immediately by flushing the buffer
            client.flush().unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
 