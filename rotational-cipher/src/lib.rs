pub fn rotate(input: &str, key: i8) -> String {
    return input.chars().map(|c| shift(c as char, key)).collect();
}

fn shift(ch: char, key: i8) -> char {
    let base = if ch.is_uppercase() { b'A' as u8 } else { b'a' as u8 };
    let key = ((key % 26 + 26) % 26) as u8;
    match ch.is_alphabetic() {
        true  => (base + ((ch as u8) - base + key) % 26) as char,
        false => ch
    }
}
