

pub fn preprocess(original: &String) -> String {
    let lowercased = original.to_lowercase();
    let no_punctuation: String = lowercased.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    return no_punctuation;
}