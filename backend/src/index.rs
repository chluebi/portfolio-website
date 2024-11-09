use std::collections::{BTreeMap, HashMap};

use crate::preprocessing::preprocess;
use crate::types::{IRSystem, Index, PreIndex, Project, ProjectEntry, ProjectMapping, QueryResult};

pub fn build_word_index(projects: Vec<Project>) -> IRSystem {
    let mut tree: PreIndex = BTreeMap::new();

    let mut mapping: ProjectMapping = HashMap::new();
    let mut project_lengths = HashMap::new();

    projects.iter().for_each(|project| {
        let mut project_term_count_map: HashMap<String, u32> = HashMap::new();

        for text in project.all_text() {
            let words = preprocess(text);
            for word in &words {
                if tree.contains_key(word) {
                    let entry_map: &mut HashMap<u32, ProjectEntry> = tree.get_mut(word).unwrap();
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

                if project_term_count_map.contains_key(word) {
                    let count_entry = project_term_count_map.get_mut(word).unwrap();
                    *count_entry += 1;
                } else {
                    project_term_count_map.insert(word.to_string(), 0);
                }
            }
        }

        let mut project_length: f32 = 0 as f32;
        for (_, count) in project_term_count_map.iter() {
            let tf = score_term_frequency(*count);
            project_length += tf * tf;
        }

        mapping.insert(project.id, project.clone());
        project_lengths.insert(project.id, project_length.sqrt());
    });

    let final_tree: Index = tree.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();

    println!("index {:?}", final_tree);
    println!("lengths {:?}", project_lengths);

    return IRSystem {index: final_tree, mapping: mapping, project_lengths: project_lengths};
}

fn score_term_frequency(tf: u32) -> f32 {
    (1 as f32) + ((1 + tf) as f32).ln()
}

fn score_document_frequency(df: usize, N: usize) -> f32 {
    (1 as f32) + ((N as f32)/(df as f32)).ln()
}

pub fn query_index(system: &IRSystem, query: String) -> Vec<QueryResult> {

    let mut scores: HashMap<u32, f32> = HashMap::new();

    for (project_id, _) in system.mapping.iter() {
        scores.insert(*project_id, 0 as f32);
    }
    

    for word in preprocess(&query) {
        match system.index.get(&word) {
            Some(res) => {
                for entry in res.iter() {
                    scores.insert(entry.id, scores.get(&entry.id).unwrap() 
                    + score_term_frequency(entry.count) * score_document_frequency(res.len(), system.mapping.len()));
                }
            }
            None => {

            }
        }
    }

    println!("scores {:?}", scores);

    let mut results = scores.into_iter().map(|(id, score)| (id, score/(*system.project_lengths.get(&id).unwrap()))).collect::<Vec<(u32, f32)>>();
    results.sort_by(|(_, score_a), (_, score_b) | score_a.partial_cmp(&score_b).unwrap());
    let results = results.iter().map(|(id, score)| QueryResult {id: id.clone(), score: score.clone() as f32}).collect();

    println!("query result {:?}", results);
    return results;
}

