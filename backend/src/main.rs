use std::env;
use std::net::TcpListener;
mod socket;
mod parse;

fn main() {
    parse::read();
}

fn run_socket() {
    let host_str = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or("5000".to_string());
    let addr = host_str + ":" + &port_str;

    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                socket::handle_client(stream);
            });
        }
    }
}
