use std::io::{Read, Write};
use prost::Message;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/portfolio.rs"));
}

pub fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        match protos::Query::decode(&buffer[..bytes_read]) {
            Ok(query) => {
                let response: protos::Projects = protos::Projects::default();

                // Send the response back with the same ID
                stream.write_all(&response.encode_to_vec()).unwrap();
            },
            Err(e) => {}
        };  
    }
}