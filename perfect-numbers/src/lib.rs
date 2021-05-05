#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    
    use std::cmp::Ordering;
    
    let aliquot: u64= 
        (1..=(num / 2))
        .filter(|i| num % i == 0)
        .fold(0, |sum, i| sum + i);

    if num != 0 {
        match aliquot.cmp(&num) {
            Ordering::Greater => 
                return Some(Classification::Abundant),
            Ordering::Equal => 
                return  Some(Classification::Perfect),
            Ordering::Less => 
                return Some(Classification::Deficient)
        }
    } 
    return None
}
