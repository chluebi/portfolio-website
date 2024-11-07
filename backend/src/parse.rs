use std::fs;
use std::io::{self, Read};
use std::path::Path;
use regex::Regex;
use serde::Deserialize;

use crate::process_files;
use crate::types::{Project};

#[derive(Debug, Deserialize)]
struct Metadata {
    url: String,
    git_url: String,
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


pub fn read_project_files() -> Vec<Project> {
    let directory_path = "../data";

    let dir = fs::read_dir(directory_path);

    match dir {
        Ok(d) => {
            let paths = d
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().unwrap().is_file())
                .map(|entry| entry.path())
                .filter(|path| !path.file_stem().unwrap().to_string_lossy().to_string().starts_with("__"))
                .collect::<Vec<_>>();

            let projects: Vec<Project> = paths.iter().enumerate().filter_map(
                |(i, path)| {
                    let mut file = match fs::File::open(&path) {
                        Ok(file) => file,
                        Err(error) => {
                            eprintln!("Error opening file: {}", error);
                            return None;
                        }
                    };
    
                    let mut contents = String::new();
                    if let Err(error) = file.read_to_string(&mut contents) {
                        eprintln!("Error reading file: {}", error);
                        return None;
                    }
    
                    let metadata_str = match extract_metadata(&contents) {
                        Some(m) => m,
                        None => {
                            eprintln!("No metadata in: {}", path.display());
                            return None;
                        }
                    };
                    let metadata: Metadata = serde_yaml::from_str(&metadata_str).unwrap();
                    let content = match extract_content(&contents) {
                        Some(c) => c,
                        None => {
                            eprintln!("No content in: {}", path.display());
                            return None;
                        }
                    };

                    let files = process_files::get_project_files(&metadata.git_url).unwrap();
                    println!("Files {}", files.len());

                    return Some (Project {
                        id: i as u32,
                        year: metadata.year,
                        title: path.file_stem().unwrap().to_string_lossy().to_string(),
                        description: content.to_string(),
                        url: metadata.url,
                        git_url: metadata.git_url,
                        languages: metadata.languages,
                        tags: metadata.keywords,
                        files: files
                    })
                }
            ).collect();

            for p in &projects {
                println!("project {}", p.title);
            }

            return projects;
        }
        Err(e) => {
            eprintln!("Error reading folder: {}", directory_path);
            return Vec::new();
        }
    }

    
}