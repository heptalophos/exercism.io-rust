#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    
    let aliquot: u64= 
        (1..=(num / 2))
        .filter(|i| num % i == 0)
        .fold(0, |sum, i| sum + i);

        if num != 0 {
            if aliquot > num {
                return Some(Classification::Abundant)
            }
            if aliquot == num {
                return  Some(Classification::Perfect)
            }
            if aliquot < num {
                return Some(Classification::Deficient)
            }
        }   
        return None
}
