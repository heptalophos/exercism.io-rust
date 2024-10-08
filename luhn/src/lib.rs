/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let sanitized = code.replace(" ", "");
    let mut checksum = 0;
    let mut count = 0;
    for (i, c) in sanitized.char_indices() {
        count += 1;
        match ((sanitized.len() - i - 1) % 2, c.to_digit(10)) {
            (0, Some(d)) => checksum += d,
            (1, Some(d)) => checksum += (d << 1) - if d > 4 { 9 } else { 0 },
            (_, _)       => return false
        }
    }
    if count < 2 { false } 
    else { checksum % 10 == 0 }
}
