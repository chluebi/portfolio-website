use std::collections::{BTreeMap, HashMap, HashSet};

use crate::preprocessing::preprocess;
use crate::types::{IRSystem, Index, PreIndex, Project, ProjectEntry, ProjectMapping, QueryResult};


pub fn build_word_index(projects: Vec<Project>) -> IRSystem {
    let mut tree: PreIndex = BTreeMap::new();

    let mut mapping: ProjectMapping = HashMap::new();
    projects.iter().for_each(|project| {
        for text in project.all_text() {
            for word in preprocess(text) {
                if tree.contains_key(&word) {
                    let entry_map: &mut HashMap<u32, ProjectEntry> = tree.get_mut(&word).unwrap();
                    match entry_map.get_mut(&project.id) {
                        Some(entry) => {
                            entry.count += 1;
                        }
                        None => {
                            entry_map.insert(project.id, ProjectEntry {id: project.id, count: 1});
                        }
                    }
                } else {
                    let mut term_map = HashMap::new();
                    term_map.insert(project.id, ProjectEntry {id: project.id, count: 1});
                    tree.insert(word.to_string(), term_map);
                }
            }
        }

        mapping.insert(project.id, project.clone());
    });

    let final_tree: Index = tree.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();

    println!("index {:?}", final_tree);

    return IRSystem {index: final_tree, mapping: mapping};
}

pub fn query_index(system: &IRSystem, query: String) -> Vec<QueryResult> {

    let mut scores: HashMap<u32, u32> = HashMap::new();

    for (project_id, _) in system.mapping.iter() {
        scores.insert(*project_id, 0);
    }
    

    for word in preprocess(&query) {
        match system.index.get(&word) {
            Some(res) => {
                for entry in res.iter() {
                    scores.insert(entry.id, scores.get(&entry.id).unwrap()+entry.count);
                }
            }
            None => {

            }
        }
    }

    println!("scores {:?}", scores);

    let mut results = scores.into_iter().collect::<Vec<(u32, u32)>>();
    results.sort_by_key(|(_, score) | score.clone());
    let results = results.iter().map(|(id, score)| QueryResult {id: id.clone(), score: score.clone()}).collect();

    println!("index {:?}", system.index);
    println!("query result {:?}", results);
    return results;
}

