pub fn brackets_are_balanced(string: &str) -> bool {
    
    let mut stack: Vec<char> = vec![];

    for ch in string.chars() {
        match ch {
            '}' | ']' | ')' 
                if Some(ch) != stack.pop() =>
                        return false,
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            _   => (),
        }
    }
    return stack.is_empty()
}
