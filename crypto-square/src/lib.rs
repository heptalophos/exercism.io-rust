pub fn encrypt(input: &str) -> String {
    let sanitized_input_vector = 
        input.chars().filter(char::is_ascii_alphanumeric)
             .map(|ch| ch.to_ascii_lowercase())
             .collect::<Vec<_>>();
        if sanitized_input_vector.is_empty() {String::new()}
        else {
            (0..square_side(sanitized_input_vector.len()))
            .map(|c| sanitized_input_vector
                     .chunks(square_side(sanitized_input_vector.len()))
                     .map(|r| r.get(c).unwrap_or(&' ')).collect::<String>())
                     .collect::<Vec<_>>().join(" ")
        }
}

fn square_side(len: usize) -> usize {
    (len as f64).sqrt().ceil() as usize
}
