use std::collections::HashSet as Set;


/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = (b'a'..=b'z').map(char::from)
                                .collect::<Set<_>>();
    let candidate = sentence
                    .to_lowercase()
                    .chars()
                    .fold(Set::new(), |mut acc, ch| {
                        if ('a'..='z').contains(&ch) {
                            acc.insert(ch);
                        }
                    acc});
    alphabet == candidate
}
