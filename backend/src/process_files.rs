use std::process::Command;
use tempfile::tempdir;
use std::fs;
use std::path::PathBuf;

use crate::types::{Project, ProjectFile};


pub fn get_project_files(repo_url: &String) -> Option<Vec<ProjectFile>> {
    let temp_dir = tempdir().unwrap();
    let temp_dir_path = temp_dir.path().to_path_buf();

    let output = Command::new("git")
        .args(["clone", repo_url, temp_dir_path.to_str().unwrap()])
        .output().unwrap();

    if output.status.success() {
        return Some(visit_files(&temp_dir_path));
    } else {
        eprintln!(
            "Failed to clone repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }
    
}

fn visit_files(path: &PathBuf) -> Vec<ProjectFile> {
    let mut results: Vec<ProjectFile> = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            results.extend(visit_files(&path));
        } else {
            match (path.file_name(), path.extension(), fs::read_to_string(path.clone())) {
                (Some(file_name), Some(extension), Ok(content)) => {
                    match extension_allowed(&extension.to_string_lossy().to_string()) {
                        Some(extension) => {
                                results.push(ProjectFile {
                                name: file_name.to_string_lossy().to_string(),
                                extension: extension.to_string(),
                                content: content
                            })
                        }
                        | _ => {}
                    }
                    
                }
                _ => {}
            }
        }
    }
    results
}


fn extension_allowed(extension: &String) -> Option<&String> {
    match extension.as_str() {
        "txt" | "md"
        | "json" | "yml" | "toml" | "nix"
        | "py" | "ml" | "rs" | "ts" | "js" | "java" | "c" | "cpp" | "h"
        | "sh"
        => Some(extension),
        _ => None
    }
}