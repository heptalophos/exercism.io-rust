pub fn score(word: &str) -> usize {
    
    let mut score = 0;
    for letter in word.to_ascii_uppercase().chars() {
        score += match letter {
            x if "AEIOULNRST".contains(x) => 1,
            x if "DG".contains(x) => 2,
            x if "BCMP".contains(x) => 3,
            x if "FHVWY".contains(x) => 4,
            x if "K".contains(x) => 5,
            x if "JX".contains(x) => 8,
            x if "QZ".contains(x) => 10,
            _ => 0,
        }
    }
    score
}
