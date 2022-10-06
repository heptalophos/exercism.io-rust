#[derive(Debug, PartialEq, Eq)]

pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    use std::cmp::Ordering;
    let aliquot: u64= (1..=(num / 2)).filter(|i| num % i == 0)
                                     .fold(0, |sum, i| sum + i);
    if num != 0 {
        return match aliquot.cmp(&num) {
            Ordering::Greater => Some(Classification::Abundant),
            Ordering::Equal   => Some(Classification::Perfect),
            Ordering::Less    => Some(Classification::Deficient)
        }
    } 
    None
}
