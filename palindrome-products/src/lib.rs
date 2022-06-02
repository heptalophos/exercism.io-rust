#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Self> {
        Some(value).filter(|x| {
            let s = format!("{:?}", x);
            s == s.chars().rev().collect::<String>()
        }).map(Palindrome)
    }
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut pmin = max * max;
    let mut pmax = min * min;
    let mut found = 0;
    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            let digits = format!("{:?}", product);
            if product <= pmin {
                if digits.chars().eq(digits.chars().rev()) {
                    pmin = i * j;
                    found += 1;
                }
            }
            if product >= pmax {
                if digits.chars().eq(digits.chars().rev()) {
                    pmax = i * j; 
                    found += 1;
                }
            }
        }
    }
    match found {
        0 => None,
        _ => Palindrome::new(pmin).zip(Palindrome::new(pmax))
    }
}
