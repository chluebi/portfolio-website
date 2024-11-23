use std::collections::{BTreeMap, HashMap};

use crate::preprocessing::preprocess;
use crate::types::{FieldWeights, IRSystem, Index, LengthStore, PreIndex, Project, ProjectEntry, ProjectMapping, QueryResult, TrigramMap};


fn add_word_to_index(word: &String, index: &mut PreIndex, term_count_map: &mut HashMap<String, u32>, project: &Project) {

    if index.contains_key(word) {
        let entry_map: &mut HashMap<u32, ProjectEntry> = index.get_mut(word).unwrap();
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
        index.insert(word.to_string(), term_map);
    }

    if term_count_map.contains_key(word) {
        let count_entry = term_count_map.get_mut(word).unwrap();
        *count_entry += 1;
    } else {
        term_count_map.insert(word.to_string(), 0);
    }
}

fn turn_term_count_into_length(term_count_map: &HashMap<String, u32>) -> f32 {
    let mut project_length: f32 = 0 as f32;
    for (_, count) in term_count_map.iter() {
        let tf = score_term_frequency(*count);
        project_length += tf * tf;
    }
    return project_length.sqrt();
}


fn get_word_trigrams(word: &str) -> Vec<String> {
    let chars: Vec<char> = format!("*{}*", word).chars().collect();
    chars.windows(3)
         .map(|window| window.iter().collect())
         .collect()
}




pub fn build_word_index(projects: Vec<Project>) -> IRSystem {
    let mut title_preindex: PreIndex = BTreeMap::new();
    let mut description_preindex: PreIndex = BTreeMap::new();
    let mut languages_preindex: PreIndex = BTreeMap::new();
    let mut tags_preindex: PreIndex = BTreeMap::new();
    let mut files_preindex: PreIndex = BTreeMap::new();

    let mut title_lengths: LengthStore = HashMap::new();
    let mut description_lengths: LengthStore = HashMap::new();
    let mut languages_lengths: LengthStore = HashMap::new();
    let mut tags_lengths: LengthStore = HashMap::new();
    let mut files_lengths: LengthStore = HashMap::new();

    let mut trigrams: TrigramMap = HashMap::new();

    let mut mapping: ProjectMapping = HashMap::new();

    projects.iter().for_each(|project| {
        let mut title_term_count_map: HashMap<String, u32> = HashMap::new();
        for word in preprocess(&project.title) {
            add_word_to_index(&word, &mut title_preindex, &mut title_term_count_map, project);
            
            if !trigrams.contains_key(&word) {
                trigrams.insert(word.clone(), get_word_trigrams(&word));
            }
        }
        title_lengths.insert(project.id, turn_term_count_into_length(&title_term_count_map));


        let mut description_term_count_map: HashMap<String, u32> = HashMap::new();
        for word in preprocess(&project.description) {
            add_word_to_index(&word, &mut description_preindex, &mut description_term_count_map, project);

            if !trigrams.contains_key(&word) {
                trigrams.insert(word.clone(), get_word_trigrams(&word));
            }
        }
        description_lengths.insert(project.id, turn_term_count_into_length(&description_term_count_map));


        let mut languages_term_count_map: HashMap<String, u32> = HashMap::new();
        for text in project.languages.iter() {
            for word in preprocess(&text) {
                add_word_to_index(&word, &mut languages_preindex, &mut languages_term_count_map, project);

                if !trigrams.contains_key(&word) {
                    trigrams.insert(word.clone(), get_word_trigrams(&word));
                }
            }
        }
        languages_lengths.insert(project.id, turn_term_count_into_length(&languages_term_count_map));


        let mut tags_term_count_map: HashMap<String, u32> = HashMap::new();
        for text in project.tags.iter() {
            for word in preprocess(&text) {
                add_word_to_index(&word, &mut tags_preindex, &mut tags_term_count_map, project);

                if !trigrams.contains_key(&word) {
                    trigrams.insert(word.clone(), get_word_trigrams(&word));
                }
            }
        }
        tags_lengths.insert(project.id, turn_term_count_into_length(&tags_term_count_map));


        let mut files_term_count_map: HashMap<String, u32> = HashMap::new();
        for file in project.files.iter() {
            for word in preprocess(&file.content) {
                add_word_to_index(&word, &mut files_preindex, &mut files_term_count_map, project);

                if !trigrams.contains_key(&word) {
                    trigrams.insert(word.clone(), get_word_trigrams(&word));
                }
            }
        }
        files_lengths.insert(project.id, turn_term_count_into_length(&files_term_count_map));


        mapping.insert(project.id, project.clone());
        
    });

    let title_index: Index = title_preindex.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();

    let description_index: Index = description_preindex.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();

    let languages_index: Index = languages_preindex.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();

    let tags_index: Index = tags_preindex.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();

    let files_index: Index = files_preindex.into_iter().map(|(key, value)| {
        let mut vec: Vec<ProjectEntry> = value.into_iter().map(|(_, item)| item.clone()).collect();
        vec.sort();
        (key, vec)
    }).collect();


    println!("title index {:?}", title_index);
    println!("files lengths {:?}", files_lengths);

    return IRSystem {
        title_index: (title_index, title_lengths),
        description_index: (description_index, description_lengths),
        languages_index: (languages_index, languages_lengths),
        tags_index: (tags_index, tags_lengths),
        files_index: (files_index, files_lengths),
        trigrams: trigrams,
        mapping: mapping
    };
}



fn score_term_frequency(tf: u32) -> f32 {
    (1 as f32) + ((1 + tf) as f32).ln()
}

fn score_document_frequency(df: usize, N: usize) -> f32 {
    (1 as f32) + ((N as f32)/(df as f32)).ln()
}


fn score_for_index(word: &String, index: &Index, scores: &mut HashMap<u32, f32>, n: usize) {
    match index.get(word) {
        Some(res) => {
            for entry in res.iter() {
                scores.insert(entry.id, scores.get(&entry.id).unwrap() 
                + score_term_frequency(entry.count) * score_document_frequency(res.len(), n));
            }
        }
        None => {

        }
    }
}


pub fn query_index(system: &IRSystem, query: String, weights: FieldWeights) -> Vec<QueryResult> {


    let mut title_scores: HashMap<u32, f32> = HashMap::new();
    let mut description_scores: HashMap<u32, f32> = HashMap::new();
    let mut languages_scores: HashMap<u32, f32> = HashMap::new();
    let mut tags_scores: HashMap<u32, f32> = HashMap::new();
    let mut files_scores: HashMap<u32, f32> = HashMap::new();

    let mut scores: HashMap<u32, f32> = HashMap::new();


    for (project_id, _) in system.mapping.iter() {
        title_scores.insert(*project_id, 0 as f32);
        description_scores.insert(*project_id, 0 as f32);
        languages_scores.insert(*project_id, 0 as f32);
        tags_scores.insert(*project_id, 0 as f32);
        files_scores.insert(*project_id, 0 as f32);

        scores.insert(*project_id, 0 as f32);
    }

    
    
    for word in preprocess(&query) {
        score_for_index(&word, &system.title_index.0, &mut title_scores, system.mapping.len());
        score_for_index(&word, &system.description_index.0, &mut description_scores, system.mapping.len());
        score_for_index(&word, &system.languages_index.0, &mut languages_scores, system.mapping.len());
        score_for_index(&word, &system.tags_index.0, &mut tags_scores, system.mapping.len());
        score_for_index(&word, &system.files_index.0, &mut files_scores, system.mapping.len());
    }

    println!("title scores {:?}", title_scores);
    println!("description scores {:?}", description_scores);
    println!("languages scores {:?}", languages_scores);
    println!("tags scores {:?}", tags_scores);
    println!("files scores {:?}", files_scores);

    for (project_id, score) in title_scores {
        scores.insert(project_id, *scores.get(&project_id).unwrap() + weights.title * score/(*system.title_index.1.get(&project_id).unwrap()));
    }

    for (project_id, score) in description_scores {
        scores.insert(project_id, *scores.get(&project_id).unwrap() + weights.description * score/(*system.description_index.1.get(&project_id).unwrap()));
    }

    for (project_id, score) in languages_scores {
        scores.insert(project_id, *scores.get(&project_id).unwrap() + weights.languages * score/(*system.languages_index.1.get(&project_id).unwrap()));
    }

    for (project_id, score) in tags_scores {
        scores.insert(project_id, *scores.get(&project_id).unwrap() + weights.tags * score/(*system.tags_index.1.get(&project_id).unwrap()));
    }

    for (project_id, score) in files_scores {
        scores.insert(project_id, *scores.get(&project_id).unwrap() + weights.files * score/(*system.files_index.1.get(&project_id).unwrap()));
    }

    println!("final scores {:?}", scores);

    let mut results: Vec<(u32, f32)> = scores.into_iter().collect();
    results.sort_by(|(_, score_a), (_, score_b) | score_b.partial_cmp(&score_a).unwrap()); // swapped for reverse
    let results = results.iter().map(|(id, score)| QueryResult {id: id.clone(), score: score.clone() as f32}).collect();

    println!("query result {:?}", results);
    return results;
}

