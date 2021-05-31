use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, 
                        possible_anagrams: &[& 'a str]) -> 
                                            HashSet<&'a str> {
    let mut word_v = word.to_lowercase()
                           .chars()
                           .collect::<Vec<char>>();
    word_v.sort_unstable();
    
    possible_anagrams
    .iter()
    .filter(|&&candidate| {
        candidate.to_lowercase() != word.to_lowercase() && {
            let mut cand_v = candidate
                             .to_lowercase()
                             .chars()
                             .collect::<Vec<char>>();
            cand_v.sort_unstable();
            word_v == cand_v
        }
    })
    .copied()
    .collect()
}
