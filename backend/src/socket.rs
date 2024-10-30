use std::io::{Read, Write};
use prost::Message;

use crate::{types, index};

mod protos {
    include!(concat!(env!("OUT_DIR"), "/portfolio.rs"));
}

pub fn handle_client(mut stream: std::net::TcpStream, system: &types::IRSystem) {
    let mut buffer = [0; 512];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        match protos::Query::decode(&buffer[..bytes_read]) {
            Ok(query) => {

                let ids = index::query_index(&system.index, query.query.split_whitespace().collect());
                let projects: Vec<types::Project> = ids.iter().filter_map(|id| system.mapping.get(id).cloned()).collect();
                let protos_projects: Vec<protos::Project> = projects.iter().map(
                    |project| {
                        return protos::Project {
                            id: project.id as i32,
                            year: project.year as i32,
                            title: project.title.clone(),
                            description: project.description.clone(),
                            url: project.url.clone(),
                            languages: project.languages.clone(),
                            tags: project.tags.clone()
                        }
                    }
                ).collect();

                let response: protos::Response = protos::Response { 
                        uuid: query.uuid,
                        projects: Some(protos::Projects {projects: protos_projects})
                    };

                // Send the response back with the same ID
                stream.write_all(&response.encode_to_vec()).unwrap();
            },
            Err(e) => {}
        };  
    }
}