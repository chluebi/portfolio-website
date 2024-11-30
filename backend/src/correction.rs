use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

use crate::types::{IRSystem, TrigramMap};


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


fn get_word_trigrams(word: &str) -> Vec<String> {
    let chars: Vec<char> = format!("*{}*", word).chars().collect();
    chars.windows(3)
         .map(|window| window.iter().collect())
         .collect()
}


pub fn find_closest_jaccard_matches(word: &String, system: &IRSystem, count: usize, prefix: &String) -> Vec<(String, f32)> {

    let word_trigrams: HashSet<String> = get_word_trigrams(word).iter().cloned().collect();
    let word_size = word_trigrams.len();

    let mut heap = BinaryHeap::with_capacity(count);

    for (other_word, other_trigrams) in system.trigrams.iter() {
        if !prefix.is_empty() && !other_word.starts_with(prefix) {
            continue;
        }
        
        let score = {
            if other_word == word {
                100.0
            } else {
                let other_size = other_trigrams.len();
                let intersection_size = other_trigrams.iter().filter(|&t| word_trigrams.contains(t)).count(); 

                (intersection_size as f32) / ((word_size + other_size - intersection_size) as f32)
                * (2.0 + *system.term_scores.get(other_word).unwrap()).ln() // scaling for more common words
                // scaling for longer suggestions
                * if !prefix.is_empty() && other_word.len() > word.len() 
                {((other_word.len() - word.len()) as f32).ln()} 
                else {1.0} 
            } 
        };

        
        

        if heap.len() < count {
            heap.push(JaccardScore{score: score, word: other_word.clone()});
        } else if let Some(smallest) = heap.peek() {
            if score > smallest.score {
                heap.pop();
                heap.push(JaccardScore{score: score, word: other_word.clone()});
            }
        }
    }

    println!("heap {:?}", heap);

    heap.into_sorted_vec().iter().map(|js| (js.word.clone(), js.score)).collect()
}


fn min_edit_distance(s1: &String, s2: &String) -> u32 {
    let l1 = s1.len();
    let l2 = s2.len();

    let mut dp = vec![vec![0 as u32; l2 + 1]; l1 + 1];

    for i in 0..l1+1 {
        dp[i][0] = i as u32;
    }

    for j in 0..l2+1 {
        dp[0][j] = j as u32;
    }

    for i in 1..l1+1 {
        for j in 1..l2+1 {
            let cost = if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                0
            } else {
                1
            };

            dp[i][j] = *[
                dp[i - 1][j] + 1,
                dp[i][j - 1] + 1,
                dp[i - 1][j - 1] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    return dp[l1][l2];
}



pub fn find_closest_match(word: &String, system: &IRSystem, sample_count: usize, prefix: &String) -> Option<(String, u32)> {
    let matches: Vec<(String, f32)> = find_closest_jaccard_matches(word, system, sample_count, prefix).iter().map(
        |x| (x.0.clone(), x.1)
    ).collect();

    let scores: Vec<i32> = matches.iter().map(|x| 
        (min_edit_distance(&word, &x.0) as i32) * 100
        - (((1.0 + *system.term_scores.get(&x.0).unwrap() as f32).ln() * 100.0) as i32) // scaling for more common words
        - // for longer suggestions
        if !prefix.is_empty() && x.0.len() > word.len() 
        {(((1 + x.0.len() - word.len()) as f32).ln() * 100.0) as i32} 
        else {0} 
    ).collect();

    if let Some(min_index) = scores.iter().enumerate().min_by_key(|&(_, score)| score) {
        Some((matches[min_index.0].0.clone(), *min_index.1.max(&0) as u32))
    } else {
        None
    }
}