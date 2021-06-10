/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {

    if isbn.replace("-", "").len() != 10 {
        return false
    }

    isbn.replace("-", "")
        .chars()
        .enumerate()
        .map(|(idx, c)| match (idx, c) {
                (9, 'X') => Ok(10),
                (_, 'X') => Err("invalid X"),
                (_,  d)  => d.to_string()
                             .parse::<usize>()
                             .or(Err("digit err")),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|d| {
           d.iter().enumerate()
            .fold(0usize, 
                  |acc, (i, n)| 
                  acc + n * (10 - i)) % 11 == 0 
        })
        .unwrap_or(false)
}
