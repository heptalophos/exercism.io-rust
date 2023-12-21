use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters = input.chars()
                       .filter(char::is_ascii_alphabetic)
                       .collect::<HashSet<char>>();
    let (tokens, right_term) = parse_terms(input);
    (0..=9).permutations(letters.len())
           .map(|ls| letters.iter().cloned()
                                   .zip(ls).collect::<HashMap<char, u8>>())
           .filter(|s| valid_solution(&tokens, right_term, s))
           .next()
}

fn parse_terms(input: &str) -> (Vec<&str>, &str) {
    let (left, right) = input.split_once(" == ").unwrap();
    // let  = sides;
    let tokens = left.split(" + ").collect::<Vec<&str>>();
    (tokens, right)
}

fn term_value(token: &str, solution: &HashMap<char, u8>)  -> u64 {
    token.chars()
         .map(|c| solution[&c] as u64)
         .fold(0, |x, y| x * 10 + y)
}

fn left_terms_value(tokens: &[&str], solution: &HashMap<char, u8>) -> u64 {
    tokens.iter()
          .map(|token| term_value(token, solution)
          )
          .fold(0, |x, y| x + y)
}

fn zero_start(token: &str, solution: &HashMap<char, u8>) -> bool {
    solution[&token.chars().next().unwrap()] == 0
}

fn valid_solution(tokens: &[&str], right: &str, solution: &HashMap<char, u8>) -> bool {
    !tokens.iter().any(|token| zero_start(token, solution)) &&
    !zero_start(right, solution) && 
    left_terms_value(tokens, solution) == term_value(right, solution)      
}
