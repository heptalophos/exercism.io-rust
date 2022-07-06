pub fn number(user_number: &str) -> Option<String> {
    let mut digits = user_number.chars().filter(|&c| c.is_digit(10))
                                        .collect::<Vec<_>>();
    if digits[0] == '1' { digits.remove(0); } 
    if digits.len() != 10 || digits[0] < '2' || digits[3] < '2' { 
        return None; 
    }
    Some (digits.into_iter().collect::<String>())
}
