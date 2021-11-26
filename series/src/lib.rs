pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![]
    }
    (0..(digits.len() - len + 1))
    .map(|x| digits[x..(x + len)].to_string()).collect()
}
