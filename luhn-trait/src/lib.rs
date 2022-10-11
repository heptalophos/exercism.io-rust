pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: std::fmt::Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = format!("{}", &self);
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
