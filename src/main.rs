use std::net::TcpListener;

fn main() {
    let address = "127.0.0.1:8080";

    let listener = TcpListener::bind(address)
        .expect("Failed to bind Harbor to the address");

    println!("Harbor is listening on http://{address}");

    let _connection = listener
        .accept()
        .expect("Failed to accept the incoming connection");

    println!("Harbor accepted its first connection.");
}