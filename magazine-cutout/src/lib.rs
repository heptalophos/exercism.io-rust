use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut m_histogram = HashMap::new();
    let mut n_histogram = HashMap::new();
    for mword in magazine {
        *m_histogram.entry(mword).or_insert(0) += 1;
    };
    for nword in note {
        *n_histogram.entry(nword).or_insert(0) += 1;
    };
    n_histogram.iter().all( &|(nword, nfreq)| 
                            m_histogram.get(nword).unwrap_or(&0) 
                            >= nfreq
                          )
}
