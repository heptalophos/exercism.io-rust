#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Palindrome {
    x: u64,
    y: u64
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {a, b}
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
    unimplemented!(
        "Find the min and max palindromic numbers which are products of numbers in the inclusive range ({}..{})",
        min,
        max
    )
}
