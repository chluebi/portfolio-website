use std::fs;
use std::io::{self, Read};
use std::path::Path;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metadata {
    url: String,
    year: u32,
    languages: Vec<String>,
    keywords: Vec<String>,
}


fn extract_metadata(content: &str) -> Option<&str> {
    let re = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();
    re.captures(content).and_then(|caps| caps.get(1).map(|m| m.as_str()))
}

fn extract_content(content: &str) -> Option<&str> {
    let re = Regex::new(r"(?s)^---\n(.*?)\n---\n(.*)").unwrap();
    re.captures(content).and_then(|caps| caps.get(2).map(|m| m.as_str()))
}


pub fn read() {
    let directory_path = "../data";

    let dir = fs::read_dir(directory_path);

    match dir {
        Ok(d) => {
            let paths = d
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().unwrap().is_file())
                .map(|entry| entry.path())
                .collect::<Vec<_>>();

            for path in paths {
                println!("Reading file: {}", path.display());

                let mut file = match fs::File::open(&path) {
                    Ok(file) => file,
                    Err(error) => {
                        eprintln!("Error opening file: {}", error);
                        continue;
                    }
                };

                let mut contents = String::new();
                if let Err(error) = file.read_to_string(&mut contents) {
                    eprintln!("Error reading file: {}", error);
                    continue;
                }

                let metadata_str = match extract_metadata(&contents) {
                    Some(m) => m,
                    None => {
                        eprintln!("No metadata in: {}", path.display());
                        continue;
                    }
                };
                let metadata: Metadata = serde_yaml::from_str(&metadata_str).unwrap();
                let content = match extract_content(&contents) {
                    Some(c) => c,
                    None => {
                        eprintln!("No content in: {}", path.display());
                        continue;
                    }
                };
                
                println!("File metadata: \n{}", metadata_str);
                println!("File content:\n{}", content);
                println!("File url: {}", metadata.url)
            }
        }
        Err(e) => {
            eprintln!("Error reading folder: {}", directory_path);
        }
    }

    
}