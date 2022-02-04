#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    x: u64, y: u64
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {x: a, y: b}
    }
    pub fn value(&self) -> u64 {
        self.x * self.y
    }
    pub fn insert(&mut self, a: u64, b: u64) {
        self.x = a;
        self.y = b;
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut pmin = Palindrome::new(max, max);
    let mut pmax = Palindrome::new(min, min);
    let mut found = 0;
    for i in min..=max {
        for j in i..=max {
            let product = Palindrome::new(i, j).value();
            let digits = format!("{}", product);
            if product <= pmin.value() {
                if digits.chars().eq(digits.chars().rev()) {
                    pmin.insert(i, j);
                    found += 1;
                }
            }
            if product >= pmax.value() {
                if digits.chars().eq(digits.chars().rev()) {
                    pmax.insert(i, j); 
                    found += 1;
                }
            }
        }
    }
    match found {
        0 => None,
        _ => Some((pmin, pmax))
    }
}
