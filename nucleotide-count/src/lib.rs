use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        Err(nucleotide)
    } else {
        dna.chars().try_fold(0, |acc, n| {
            match ("ACGT".contains(n), n == nucleotide) {
                (true, true) => Ok(acc + 1),
                (true, false) => Ok(acc),
                (_, _) => Err(n),
            }
        })
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}
