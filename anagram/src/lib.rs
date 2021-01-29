use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, 
                        possible_anagrams: &[& 'a str]) -> 
                                        HashSet<&'a str> {
    possible_anagrams.iter()
                     .filter(|&w| is_anagram(w, word))
                     .copied()
                     .collect()
}

fn is_anagram(candidate: &str, word: &str) -> bool {
    if candidate.len() == word.len() {
        let mut cand_v =
                candidate.to_lowercase()
                         .chars()
                         .collect::<Vec<char>>();
        let mut word_v =
                word.to_lowercase()
                    .chars()
                    .collect::<Vec<char>>();
        cand_v.sort();
        word_v.sort();
        return candidate.to_lowercase() != word.to_lowercase() 
               && cand_v == word_v;
    }
    return false
}
