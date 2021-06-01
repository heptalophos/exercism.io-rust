use std::collections::HashSet as Set;

pub fn is_pangram(sentence: &str) -> bool {

    (b'a'..=b'z').map(char::from).collect::<Set<_>>() ==
    sentence.to_lowercase()
            .chars()
            .fold(Set::new(), 
                  |mut acc, ch| { 
                        if ('a'..='z').contains(&ch) 
                            { acc.insert(ch); }
                  acc})
}
