use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    
    let mut counts: HashMap<char, u16> = HashMap::new();

    candidate.to_lowercase()
             .chars()
             .filter(|x| x.is_alphabetic())
             .for_each(|x| *counts.entry(x).or_insert(0) += 1);
    
    counts.iter().find(|(_, c)| *c > &1u16).is_none()
}          
