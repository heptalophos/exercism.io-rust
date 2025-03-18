fn protein(codon: &str) -> Option<&str> {
    match codon {
        "AUG"         => Some("Methionine"),
        "UUU" | "UUC" => Some("Phenylalanine"),
        "UUA" | "UUG" => Some("Leucine"),
        "UCU" | "UCC" | 
        "UCA" | "UCG" => Some("Serine"),
        "UAU" | "UAC" => Some("Tyrosine"),
        "UGU" | "UGC" => Some("Cysteine"),
        "UGG"         => Some("Tryptophan"),
        "UAA" | "UAG" |
        "UGA"         => Some("STOP"),
        _             => None,
    }
}

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.as_bytes().chunks(3)
       .filter_map(|nuc| Some(std::str::from_utf8(nuc).ok()))
       .map(|codon| codon.and_then(protein))
       .map_while(|p| match p {
            Some("STOP") => None,
            Some(p) => Some(Some(p)),
            None => Some(None),
           
       })
       .collect::<Option<Vec<_>>>()
}


