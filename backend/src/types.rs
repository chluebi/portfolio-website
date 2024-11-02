use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u32,
    pub year: u32,
    pub title: String,
    pub description: String,
    pub url: String,
    pub git_url: String,
    pub languages: Vec<String>,
    pub tags: Vec<String>
}

impl Project {
    pub fn all_text(&self) -> Vec<&String> {
        let mut text = vec![&self.title, &self.description, &self.url];
        let borrowed_languages: Vec<&String> = self.languages.iter().collect();
        let borrowed_tags: Vec<&String> = self.tags.iter().collect();
        text.extend(borrowed_languages);
        text.extend(borrowed_tags);
        text
    }
}

pub type PreIndex = BTreeMap<String, HashSet<u32>>;
pub type Index = BTreeMap<String, Vec<u32>>;
pub type ProjectMapping = HashMap<u32, Project>;

pub struct IRSystem {
    pub index: Index,
    pub mapping: ProjectMapping
}