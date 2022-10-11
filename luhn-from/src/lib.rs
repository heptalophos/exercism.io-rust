pub struct Luhn(String);

impl<T: std::fmt::Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(format!("{}", input))
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = &self.0;
        let sanitized = code.replace(" ", "");
        let mut checksum = 0;
        let mut count = 0;
        for (i, c) in sanitized.char_indices() {
            count += 1;
            match ((sanitized.len() - i - 1) % 2, c.to_digit(10)) {
                (0, Some(d))          => checksum +=  d,
                (1, Some(d)) if d > 4 => checksum += (d << 1) - 9,
                (1, Some(d))          => checksum +=  d << 1,
                (_, _)                => return false
            }
        }
        if count < 2 { false } 
        else { checksum % 10 == 0 }
    }
}
