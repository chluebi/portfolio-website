use std::io::{Read, Write};
use prost::Message;

use crate::{types, index, preprocessing, correction};

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
                match query.r#type {
                    0 => { // protos::QueryType::Projects
                        let results = index::query_index(&system, query.query, types::FieldWeights {title: 0.5, description: 0.2, languages: 0.4, tags: 0.3, files: 0.1});
                        let projects: Vec<types::Project> = results.iter().filter_map(|result| system.mapping.get(&result.id).cloned()).collect();
                        let protos_projects: Vec<protos::Project> = projects.iter().map(
                            |project| {
                                return protos::Project {
                                    id: project.id as i32,
                                    year: project.year as i32,
                                    title: project.title.clone(),
                                    description: project.description.clone(),
                                    url: project.url.clone(),
                                    git_url: project.git_url.clone(),
                                    languages: project.languages.clone(),
                                    tags: project.tags.clone()
                                }
                            }
                        ).collect();

                        let response: protos::Response = protos::Response { 
                                uuid: query.uuid,
                                payload: Some(protos::response::Payload::Projects(protos::Projects {projects: protos_projects}))
                            };

                        // Send the response back with the same ID
                        stream.write_all(&response.encode_to_vec()).unwrap();
                    }
                    _ => { // protos::QueryType::Completion
                        let closest_match = correction::find_closest_match(&query.query, &system, 30, &query.query);

                        match closest_match {
                            Some((closest_match, _)) => {
                                let response: protos::Response = protos::Response {
                                    uuid: query.uuid,
                                    payload: Some(protos::response::Payload::Completion(closest_match))
                                };
        
                                // Send the response back with the same ID
                                stream.write_all(&response.encode_to_vec()).unwrap();
                            }
                            None => {
                                let response: protos::Response = protos::Response {
                                    uuid: query.uuid,
                                    payload: Some(protos::response::Payload::Completion("".to_string()))
                                };
                                stream.write_all(&response.encode_to_vec()).unwrap();
                            }
                        }
                    }
                }
            },
            Err(e) => {}
        };  
    }
}