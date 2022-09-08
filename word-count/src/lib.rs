use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let quote = '\'';
    let boundary = |ch: char| ch.is_whitespace() ||
                    ch.is_ascii_punctuation() && ch != quote; 
    
    words.to_lowercase().split(boundary)
         .fold(HashMap::new(), |mut count, word| {
            if !word.is_empty() {
                *count.entry(word.trim_matches(quote).to_string())
                      .or_insert(0) += 1;
            }
            count
         })
}
