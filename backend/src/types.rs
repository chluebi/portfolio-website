use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct ProjectFile {
    pub name: String,
    pub extension: String,
    pub content: String
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: u32,
    pub year: u32,
    pub title: String,
    pub description: String,
    pub url: String,
    pub git_url: String,
    pub languages: Vec<String>,
    pub tags: Vec<String>,
    pub files: Vec<ProjectFile>
}

impl Project {
    pub fn all_text(&self) -> Vec<&String> {
        let mut text = vec![&self.title, &self.description, &self.url];
        let borrowed_languages: Vec<&String> = self.languages.iter().collect();
        let borrowed_tags: Vec<&String> = self.tags.iter().collect();
        let files_text: Vec<&String> = self.files.iter().map(|project| &project.content).collect();
        text.extend(borrowed_languages);
        text.extend(borrowed_tags);
        text.extend(files_text);
        text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectEntry {
    pub id: u32,
    pub count: u32
}

impl PartialOrd for ProjectEntry {
    fn partial_cmp(&self, other: &ProjectEntry) -> std::option::Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for ProjectEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

pub type PreIndex<'a> = BTreeMap<String, HashMap<u32, ProjectEntry>>;
pub type Index = BTreeMap<String, Vec<ProjectEntry>>;
pub type ProjectMapping = HashMap<u32, Project>;

pub struct IRSystem {
    pub index: Index,
    pub mapping: ProjectMapping
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub id: u32,
    pub score: u32
}