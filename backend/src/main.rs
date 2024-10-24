use std::net::TcpListener;
mod socket;

fn main() {
    let listener = TcpListener::bind("localhost:5000").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                socket::handle_client(stream);
            });
        }
    }
}
