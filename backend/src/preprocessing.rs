

pub fn preprocess(original: &String) -> Vec<String> {
    let lowercased = original.to_lowercase();
    let separators = &['.', ':', ',', '_'];  // Add more separators as needed

    lowercased.split(|c| separators.contains(&c) || !c.is_alphanumeric())
              .map(|word| word.to_string())
              .filter(|word| !word.is_empty())
              .collect()
}