use std::collections::HashMap;

const STOP: &str = "stop codon";

pub struct CodonsInfo<'a> (HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> 
    Option<&'a str> {
        self.0
            .get(codon)
            .copied()
    }

    pub fn of_rna(&self, rna: &str) -> 
    Option<Vec<&'a str>> {
        rna.as_bytes()
           .chunks(3)
           .filter_map(|nuc| 
                       Some(std::str::from_utf8(nuc).ok()))
           .map(|codon| self.name_for(codon.unwrap()))
           .take_while(|&codon| 
                       codon.is_none() || codon != Some(STOP))
           .collect::<Option<Vec<_>>>()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) ->
CodonsInfo<'a> {
    CodonsInfo (
        pairs.iter()
             .copied()
             .collect(),
    )
}
