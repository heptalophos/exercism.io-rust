use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let quote = '\'';
    let boundary = |ch: char| ch.is_whitespace() ||
                    ch.is_ascii_punctuation() && ch != quote; 
    
    words.trim_matches(quote)
         .split(boundary)
         .fold(HashMap::new(), 
            |mut count, word| {
                if !word.is_empty() {
                    let sanitized = word.to_lowercase()
                                                .trim_matches(quote)
                                                .to_string();
                *count.entry(sanitized).or_insert(0) += 1;
            }
            count
         })
}
