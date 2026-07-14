mod request;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let address = "127.0.0.1:8080";

    let listener = TcpListener::bind(address)
        .expect("Failed to bind Harbor to the address");

    println!("Harbor is listening on http://{address}");

    let (mut stream, client_address) = listener
        .accept()
        .expect("Failed to accept the incoming connection");

    println!("Connection accepted from {client_address}");

    let mut buffer = [0_u8; 1024];

    let bytes_read = stream
        .read(&mut buffer)
        .expect("Failed to read data from the connection");

    let request_text = String::from_utf8_lossy(&buffer[..bytes_read]);

    let (method, path, version) =
        request::parse_request_line(&request_text);

    println!("Method: {method}");
    println!("Path: {path}");
    println!("Version: {version}");

    let body = "Hello from Harbor!";

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
Content-Type: text/plain\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
        body.len(),
        body
    );

    stream
        .write_all(response.as_bytes())
        .expect("Failed to send the HTTP response");
}