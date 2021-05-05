pub fn brackets_are_balanced(string: &str) -> bool {
    
    let mut stack: Vec<char> = Vec::new();

    for ch in string.chars() {
        match ch {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '}' | ']' | ')' => { 
                let p = stack.pop();
                if p == None || p.unwrap() != ch { 
                    return false 
                }
            }, 
             _  => (),
        }
    }
    return stack.is_empty()
}
