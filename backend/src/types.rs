use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Project {
    pub id: u32,
    pub year: u32,
    pub title: String,
    pub description: String,
    pub url: String,
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

pub type Index = BTreeMap<String, Vec<u32>>;
pub type ProjectMapping = HashMap<u32, Project>;