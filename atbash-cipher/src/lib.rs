/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    chunkify(&shift(plain), 5)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    shift(cipher)
}

fn shift(text: &str) -> String {
    text.chars()
        .map(|ch| {
            if ch.is_ascii_alphabetic() {
                Some(
                    (b'z' - (ch.to_ascii_lowercase() as u8) + b'a') 
                    as char
                )
            } 
            else if ch.is_numeric() {
                Some(ch)
            }
            else {
                None
            }
        })
        .filter(|ch| !ch.is_none())
        .map(|ch| ch.unwrap())
        .collect()
    
}

fn chunkify(text: &str, chunks_of: u8) -> String {
    let mut index = 0;
    text.chars()
        .into_iter()
        .fold("".to_owned(), |mut output, ch| {
            if index > 0 && index % chunks_of == 0 {
                output.push(' ');
            }
            output.push(ch);
            index += 1;
            output
        })
}
