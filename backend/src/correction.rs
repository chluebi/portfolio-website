use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

use crate::types::{TrigramMap};


#[derive(Clone, Debug, PartialEq)]
struct JaccardScore {
    score: f32,
    word: String
}


impl Eq for JaccardScore {}

impl Ord for JaccardScore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.partial_cmp(&self.score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for JaccardScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



pub fn find_closest_jaccard_matches(word: &String, trigrams_map: &TrigramMap, count: usize) -> Vec<(String, f32)> {

    let word_trigrams: HashSet<String> = get_word_trigrams(word).iter().cloned().collect();
    let word_size = word_trigrams.len();

    let mut heap = BinaryHeap::with_capacity(count);

    for (other_word, other_trigrams) in trigrams_map.iter() {
        let other_size = other_trigrams.len();
        let intersection_size = other_trigrams.iter().filter(|&t| word_trigrams.contains(t)).count();
        let score: f32 = (intersection_size as f32) / ((word_size + other_size - intersection_size) as f32);
        

        if heap.len() < count {
            heap.push(JaccardScore{score: score, word: other_word.clone()});
        } else if let Some(smallest) = heap.peek() {
            if score > smallest.score {
                heap.pop();
                heap.push(JaccardScore{score: score, word: other_word.clone()});
            }
        }
    }

    heap.into_sorted_vec().iter().map(|js| (js.word.clone(), js.score)).collect()
}