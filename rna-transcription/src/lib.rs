#[derive(Debug, PartialEq)]
pub struct Dna {
    deoxyribonucleic_strand: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    ribonucleic_strand: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, dorn) in dna.chars().enumerate() {
            match dorn {
                'A' | 'C' | 'G' | 'T' => {}
                _ => return Err(index),
            }
        }
        Ok(Dna {
            deoxyribonucleic_strand: dna.chars().as_str().to_string(),
        })    
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            ribonucleic_strand: self
                                .deoxyribonucleic_strand
                                .chars()
                                .map (|n| match n {
                                    'G' => 'C',
                                    'C' => 'G',
                                    'T' => 'A',
                                    'A' => 'U',
                                    _ => '$',
                                })
                                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, rn) in rna.chars().enumerate() {
            match rn {
                'A' | 'C' | 'G' | 'U' => {}
                _ => return Err(index),
            }
        }
        Ok(Rna {
            ribonucleic_strand: rna.chars().as_str().to_string(),
        })     
    }
}
