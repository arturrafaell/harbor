use std::io::Read;
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

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Bytes received: {bytes_read}");
    println!("----- Raw request -----");
    println!("{request}");
    println!("-----------------------");
}