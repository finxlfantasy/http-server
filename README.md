[![progress-banner](https://backend.codecrafters.io/progress/http-server/8c9181f6-a64f-49ab-bb6a-1852e69e5327)](https://app.codecrafters.io/users/finxlfantasy?r=2qF)

HTTP Server in Rust
This repository contains a simple HTTP server implemented in Rust. The server can handle incoming requests, parse them, and generate appropriate responses. Below are the key components and functionalities of the server.

Getting Started
Prerequisites: Make sure you have Rust installed on your system. If not, you can download it from here.

Clone the Repository:

git clone https://github.com/your-username/http-server-rust.git
cd http-server-rust

Build and Run:

cargo build
cargo run

The server will start listening on port 8080 by default.

Components
1. Request Parsing
The server parses incoming HTTP requests into three main components:

Start Line: Extracts the request method, request path, and HTTP version.
Headers: Parses headers into a HashMap for easy access.
Body: Retrieves the body content (if any).
2. handle_connection Function
The heart of the server lies in the handle_connection function:

Rust
AI-generated code. Review and use carefully. More info on FAQ.

fn handle_connection(mut stream: TcpStream) {
    // Implementation details...
}
This function is called for each incoming connection. It reads the request, processes it, and generates an appropriate response.

3. Response Generation
The server currently provides a basic response for requests:

For the root path (“/”), it returns an “HTTP 200 OK” response.
For paths starting with “/echo/”, it echoes back the requested path.
Usage
Open your web browser and navigate to http://localhost:8080.
Send requests to http://localhost:8080/your-path.
Feel free to customize this server for your specific use case!
