use itertools::Itertools;
use std::{collections::{HashMap, BTreeSet}, ops::Add};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let letters = input.chars()
                       .filter(char::is_ascii_alphabetic)
                       .collect::<BTreeSet<char>>();
    let (tokens, right_term) = parse_terms(input);
    (0..=9).permutations(letters.len())
           .map(|ls| letters.iter().cloned()
                                   .zip(ls).collect::<HashMap<char, u8>>())
           .filter(|s| valid_solution(&tokens, right_term, s))
           .next() 
}

fn parse_terms(input: &str) -> (Vec<&str>, &str) {
    let sides = input.split_once("==").unwrap();
    let (left, right) = sides;
    let tokens = left.split(" + ").collect::<Vec<&str>>();
    (tokens, right)
}

fn evaluate_right(right: &str, solution: &HashMap<char, u8>)  -> u64 {
    right.chars()
         .map(|c| solution[&c] as u64)
         .fold(0, |x, y| x * 10 + y)
}

fn evaluate_terms(tokens: &[&str], solution: &HashMap<char, u8>) -> u64 {
    tokens.iter()
          .map(|token| 
                    // token.chars()
                    //      .map(|c| solution[&c] as u64)
                    //      .fold(0, |x, acc| x * 10 + acc)
                    evaluate_right(token, solution)
          )
        //   .fold(0, |x, y| x + y)
        .fold(0, Add::add)
}

fn zero_start(token: &str, solution: &HashMap<char, u8>) -> bool {
    solution[&token.chars().next().unwrap()] == 0
}

fn valid_solution(tokens: &[&str], right: &str, solution: &HashMap<char, u8>) -> bool {
    !tokens.iter().any(|token| zero_start(token, solution)) &&
    !zero_start(right, solution) && 
    evaluate_terms(tokens, solution) == evaluate_right(right, solution)        
}
// use itertools::Itertools;
// use std::collections::{BTreeSet, HashMap};
// use std::ops::Add;
// fn parse_problem<'a>(input: &'a str) -> (Vec<&'a str>, &'a str) {
//     let sides: Vec<_> = input.split(" == ").collect();
//     let lhs = sides[0];
//     let rhs = sides[1];
//     let terms: Vec<_> = lhs.split(" + ").collect();
//     (terms, rhs)
// }
// fn initial_zero(term: &str, solution: &HashMap<char, u8>) -> bool {
//     solution[&term.chars().next().unwrap()] == 0
// }
// fn eval_term(term: &str, solution: &HashMap<char, u8>) -> u64 {
//     term.chars()
//         .map(|c| solution[&c] as u64)
//         .fold(0, |x, y| x * 10 + y)
// }
// fn eval_terms(terms: &[&str], solution: &HashMap<char, u8>) -> u64 {
//     terms
//         .iter()
//         .map(|term| eval_term(term, solution))
//         .fold(0, Add::add)
// }
// fn is_solution(terms: &[&str], rhs: &str, solution: &HashMap<char, u8>) -> bool {
//     !terms.iter().any(|term| initial_zero(term, solution))
//         && !initial_zero(rhs, solution)
//         && eval_terms(terms, solution) == eval_term(rhs, solution)
// }
// pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
//     let letters: BTreeSet<char> = input.chars().filter(char::is_ascii_alphabetic).collect();
//     let (terms, rhs) = parse_problem(input);
//     (0..=9)
//         .permutations(letters.len())
//         .map(|xs| {
//             letters
//                 .iter()
//                 .cloned()
//                 .zip(xs)
//                 .collect::<HashMap<char, u8>>()
//         })
//         .filter(|solution| is_solution(&terms, rhs, solution))
//         .next()
// }
