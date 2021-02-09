use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid = |x| if "ACGT".contains(x) {1} else {0};
    let exact = |x, y| if x == y {1} else {0};

    if valid(nucleotide) != 0 {
        dna.chars().try_fold(0, |acc, n| {
            match (valid(n), exact(nucleotide, n)) {
                    (1, 1) => Ok(acc + 1),
                    (1, 0) => Ok(acc),
                    (_, _) => Err(n),
            }
        })
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    "ACGT".chars()
          .map(|n| Ok((n, count(n, dna)?)))
          .collect()
}
