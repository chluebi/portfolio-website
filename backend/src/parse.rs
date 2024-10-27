use std::fs;
use std::io::{self, Read};
use std::path::Path;

use serde_json::{Error, Result};


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

                println!("File contents:\n{}", contents);
            }
        }
        Err(e) => {
            eprintln!("Error reading folder: {}", directory_path);
        }
    }

    
}