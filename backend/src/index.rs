use std::collections::{BTreeMap, HashMap, HashSet};

use crate::preprocessing::preprocess;
use crate::types::{Project, PreIndex, Index, ProjectMapping, IRSystem};


pub fn build_word_index(projects: Vec<Project>) -> IRSystem {
    let mut tree: PreIndex = BTreeMap::new();

    let mut mapping: ProjectMapping = HashMap::new();
    projects.iter().for_each(|project| {
        for text in project.all_text() {
            for word in preprocess(text) {
                if tree.contains_key(&word) {
                    tree.get_mut(&word).unwrap().insert(project.id);
                } else {
                    tree.insert(word.to_string(), [project.id].into_iter().collect());
                }
            }
        }

        mapping.insert(project.id, project.clone());
    });

    let final_tree: Index = tree.into_iter().map(|(key, value)| {
        let mut vec: Vec<u32> = value.into_iter().collect();
        vec.sort();
        (key, vec)
    }).collect();

    println!("index {:?}", final_tree);

    return IRSystem {index: final_tree, mapping: mapping};
}

pub fn query_index(index: &Index, query: String) -> Vec<u32> {

    let mut results = HashSet::new();
    let mut first_added = false;

    for word in preprocess(&query) {
        println!("word {}", word);
        match index.get(&word) {
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

