use std::collections::BTreeMap;

use crate::types::{Project, Index, ProjectMapping};


pub fn build_word_index(projects: Vec<Project>) -> Index {
    let mut tree: Index = BTreeMap::new();
    projects.iter().for_each(|project| {
        for text in project.all_text() {
            for word in text.split_whitespace() {
                if tree.contains_key(word) {
                    tree.get_mut(word).unwrap().push(project.id);
                } else {
                    tree.insert(word.to_string(), vec![project.id]);
                }
            }
        }
    });
    println!("Tree {:?}", tree);
    return tree;
}

