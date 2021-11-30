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
    let mut pmin = None;
    let mut pmax = None;
    for i in (min..=max).into_iter() {
        for j in (i..=max).into_iter() {
            let digits = format!("{}", Palindrome::new(i, j).value());
            if digits.chars().eq(digits.chars().rev()) {
                pmin = pmin.filter(|p: &Palindrome| p.value() < (i * j))
                           .or_else(|| Some(Palindrome::new(i, j)));
                pmax = pmax.filter(|p: &Palindrome| p.value() > (i * j))
                           .or_else(|| Some(Palindrome::new(i, j)));
            }
        }
    }
    if pmin.is_none() { None }
    else { Some((pmin.unwrap(), pmax.unwrap())) }
}