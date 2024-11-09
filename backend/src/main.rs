use std::env;
use std::net::TcpListener;
use std::sync::Arc;

use types::{IRSystem};
use index::{build_word_index, query_index};
mod socket;
mod parse;
mod index;
mod types;
mod preprocessing;
mod process_files;

fn main() {
    let projects = parse::read_project_files();
    let system = Arc::new(build_word_index(projects));
    // test query
    query_index(&system, "db sqlalchemy python".to_string());
    run_socket(system);
}

fn run_socket(index: Arc<IRSystem>) {
    let host_str = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or("5000".to_string());
    let addr = host_str + ":" + &port_str;

    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let index = Arc::clone(&index);
            std::thread::spawn(move || {
                socket::handle_client(stream, &index);
            });
        }
    }
}
