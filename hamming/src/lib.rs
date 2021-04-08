/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.

pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    
    match s1.len() != s2.len() {
        true  => 
                None,
        false => 
                Some( s1.chars()
                        .zip(s2.chars())
                        .fold(0, |h, (n1, n2)| {
                               match n1 == n2 {
                                        true  => h,
                                        false => h + 1 }}))
    }
}
