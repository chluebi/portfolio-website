use std::env;
use std::net::TcpListener;
use std::sync::Arc;

use types::{FieldWeights, IRSystem};
use index::{build_word_index, query_index};
use correction::{find_closest_match};
mod socket;
mod parse;
mod index;
mod types;
mod preprocessing;
mod process_files;
mod correction;

fn main() {
    let projects = parse::read_project_files();
    let system = Arc::new(build_word_index(projects, &types::FieldWeights {title: 1.0, description: 0.2, languages: 0.4, tags: 0.3, files: 0.00001}));
    // test query
    query_index(&system, "bython abc wow".to_string(), FieldWeights {title: 0.5, description: 0.2, languages: 0.4, tags: 0.3, files: 0.1});
    let closest_match = find_closest_match(&"pyaathonic".to_string(), &system, 20, &"pyt".to_string());
    println!("closest match {:?}", closest_match.unwrap());

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
