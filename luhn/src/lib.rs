/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let sanitized = code.replace(" ", "");
    let mut checksum = 0;
    let mut count = 0;
    for (i, c) in sanitized.char_indices() {
        count += 1;
        match ((sanitized.len() - i - 1) % 2, c.to_digit(10)) {
            (0, Some(d))          => checksum += d,
            (1, Some(d)) if d > 4 => checksum += 2 * d - 9,
            (1, Some(d))          => checksum += 2 * d,
            (_, _)                => return false
        }
    }
    if count < 2 { false } 
    else { checksum % 10 == 0 }
}
