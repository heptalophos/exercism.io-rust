pub fn reverse(input: &str) -> String {

    let mut input = String::from(input);
    let mut output = String::new();

    while !input.is_empty() {
        output.push(input.pop().unwrap());    
    }
    
    output
}
