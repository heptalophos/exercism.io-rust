use std::cmp::max;
use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    let right = |c, k| (((c as u8) - b'a' + (k as u8) - b'a') % 26 + b'a') as char;
    return shift(key, s, right);
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let left = |c, k| ((26 + (c as u8) - (k as u8)) % 26 + b'a') as char;
    return shift(key, s, left);
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = (0..max(s.chars().count(), 100))
                      .map(|_| rng.gen_range(b'a'..=b'z') as char)
                      .collect::<String>();
    return (key.to_owned(), encode(&key, s).unwrap());
}

fn shift(key: &str, text: &str, rotation: fn(u8, u8) -> char) -> Option<String> {
    if key.len() <= 0 || key.chars().any(|k| !k.is_ascii_lowercase()) {
        return None
    };
    let mut shifted: Vec<char> = vec![];
    for (t, k) in text.chars().zip(key.chars().cycle()) {
        shifted.push(rotation(t as u8, k as u8));
    }
    return Some(shifted.iter().collect());
}