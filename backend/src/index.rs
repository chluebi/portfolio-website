use std::collections::{BTreeMap, HashMap, HashSet};

use crate::types::{Project, Index, ProjectMapping, IRSystem};


pub fn build_word_index(projects: Vec<Project>) -> IRSystem {
    let mut tree: Index = BTreeMap::new();
    let mut mapping: ProjectMapping = HashMap::new();
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

        mapping.insert(project.id, project.clone());
    });
    return IRSystem {index: tree, mapping: mapping};
}

pub fn query_index(index: &Index, query: String) -> Vec<u32> {

    let mut results = HashSet::new();
    let mut first_added = false;

    for word in query.split_whitespace() {
        match index.get(word) {
            Some(res) => {
                if !first_added {
                    results = res.into_iter().collect();
                    first_added = true;
                } else {
                    let mut new_set = HashSet::new();
                    for r in res {
                        new_set.insert(r);
                    }
                    results = results.intersection(&new_set).copied().collect();
                }
            }
            none => {

            }
        }
        
    }
    println!("index {:?}", index);
    println!("query result {:?}", results);
    return results.into_iter().copied().collect();
}

